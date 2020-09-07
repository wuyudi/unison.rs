const key = (o) => (typeof o === 'string' ? o : Object.keys(o)[0]);

export const patternMatch = (pattern, term) => {
    const pt = key(pattern);
    const vt = key(term);
    if (pt === 'Unbound') {
        return [];
    }
    if (pt === 'Var') {
        return [term];
    }
    if (pt === 'As') {
        const res = patternMatch(pattern[pt], value);
        return res != null ? [value].concat(res) : null;
    }
    // Boolean, Int, Nat, Float, Text, Char
    if (pt === vt) {
        return pattern[pt] === value[vt];
    }
    const bothKey = `${pt}:${vt}`;
    if (matchers[bothKey]) {
        return matchers[bothKey](pattern[pt], value[vt]);
    }
    return null;
};

const matchers = {
    'EffectBind:RequestWithContinuation': (
        [reference, number, args, kont],
        [tref, tnum, targs, tidx, tkont, current_idx],
    ) => {
        if (
            compare(reference, tref) === 0 &&
            number === tnum &&
            args.length === targs.length
        ) {
            const all = [];
            for (let i = 0; i < args.length; i++) {
                const res = patternMatch(args[i], targs[i]);
                if (res != null) {
                    all.push(...res);
                } else {
                    return null;
                }
            }
            const kk = key(kont);
            if (kk === 'Var') {
                tkont = tkont.slice(0, current_idx + 1);
                tkont[current_idx].handler = null;
                all.push({ Continuation: [tidx, tkont] });
            } else if (kk !== 'Unbound') {
                throw new Error('Unable to match on a continuation');
            }
            return all;
        } else {
            return null;
        }
    },
    'SequenceLiteral:Sequence': (patterns, items) => {
        if (patterns.length === items.length) {
            const all = [];
            const failed = patterns.some((p, i) => {
                const res = patternMatch(p, items[i]);
                if (res != null) {
                    all.push(...res);
                } else {
                    return true;
                }
            });
            if (failed) {
                return null;
            }
            return all;
        }
    },
    'SequenceOp:Sequence': ([one, op, two], contents) => {
        if (op === 'Cons') {
            if (contents.length > 0) {
                const res = patternMatch(one, contents[0]);
                if (res != null) {
                    const r2 = patternMatch(two, contents.slice(1));
                    if (r2 != null) {
                        res.concat(...r2);
                        return res;
                    }
                }
            }
            return null;
        } else if (op === 'Snoc') {
            if (contents.length > 0) {
                const res = patternMatch(one, contents.slice(0, -1));
                if (res != null) {
                    const r2 = patternMatch(two, contents[contents.length - 1]);
                    if (r2 != null) {
                        res.concat(...r2);
                        return res;
                    }
                }
            }
            return null;
        } else if (op === 'Concat') {
            const ko = key(one);
            const kt = key(two);
            if (ko === 'SequenceLiteral') {
                const patterns = one[ko];
                if (contents.length >= patterns.length) {
                    const res = patternMatch(one, {
                        Sequence: contents.slice(0, patterns.length),
                    });
                    if (res != null) {
                        const r2 = patternMatch(two, {
                            Sequence: contents.slice(patterns.length),
                        });
                        if (r2 != null) {
                            res.push(...r2);
                            return res;
                        }
                    }
                }
                return null;
            }
            if (kt === 'SequenceLiteral') {
                const patterns = two[kt];
                if (contents.length >= patterns.length) {
                    const split = contents.length - patterns.length;
                    const res = patternMatch(one, {
                        Sequence: contents.slice(0, split),
                    });
                    if (res != null) {
                        const r2 = patternMatch(two, {
                            Sequence: contents.slice(split),
                        });
                        if (r2 != null) {
                            res.push(...r2);
                            return res;
                        }
                    }
                }
                return null;
            }
            throw new Error('Concat needs sequence literal');
        }
    },
};

const matchConstructor = ([reference, number, children], inner) => {
    const all = [];
    const k = key(inner);
    if (children.length > 0) {
        if (k === 'PartialConstructor') {
            const [r, n, pchildren] = inner[k];
            if (compare(r, reference) == 0 && number === n) {
                if (pchildren.length !== children.length) {
                    return null;
                }
                for (let i = 0; i < children.length; i++) {
                    const res = patternMatch(children[i], pchildren[i]);
                    if (res != null) {
                        all.push(...res);
                    }
                }
                return all;
            }
        }
    } else {
        if (k === 'Constructor') {
            const [r, nn] = inner[k];
            if (compare(r, reference) == 0 && number === nn) {
                return all;
            }
        }
    }
    return null;
};
