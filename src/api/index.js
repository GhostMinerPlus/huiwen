import config from "./config.js";

const atom_method = {
    add: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) + new Number(arg_v[1])).toString();
        },
        len: 2
    },
    minus: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) - new Number(arg_v[1])).toString();
        },
        len: 2
    },
    mul: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) * new Number(arg_v[1])).toString();
        },
        len: 2
    },
    div: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) / new Number(arg_v[1])).toString();
        },
        len: 2
    },
    fact: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) % new Number(arg_v[1])).toString();
        },
        len: 2
    },
    g: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) > new Number(arg_v[1])).toString();
        },
        len: 2
    },
    s: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) < new Number(arg_v[1])).toString();
        },
        len: 2
    },
    eq: {
        fn: async arg_v => {
            return (arg_v[0] === arg_v[1]).toString();
        },
        len: 2
    },
    ne: {
        fn: async arg_v => {
            return (arg_v[0] !== arg_v[1]).toString();
        },
        len: 2
    },
    ge: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) >= new Number(arg_v[1])).toString();
        },
        len: 2
    },
    se: {
        fn: async arg_v => {
            return (new Number(arg_v[0]) >= new Number(arg_v[1])).toString();
        },
        len: 2
    },
    split: {
        fn: async arg_v => {
            const s_v = arg_v[0].split(arg_v[1]);
            let mp = {};
            for (let i = 0; i < s_v.length; i += 1) {
                mp[i] = s_v[i];
            }
            return mp;
        },
        len: 2
    },
    for: {
        fn: async arg_v => {
            const mp = arg_v[0];
            let r = null;
            const len = new Number(await atom_method.len.fn([mp]));
            for (let i = 0; i < len; i += 1) {
                r = await this.execute(mp[i]);
            }
            return r;
        },
        len: 1
    },
    len: {
        fn: async arg_v => {
            const mp = arg_v[0];
            return Object.keys(mp).len().toString();
        },
        len: 1
    },
    rand: {
        fn: async arg_v => {
            return Math.random().toString();
        },
        len: 1
    },
    match: {
        fn: async arg_v => {
            let left = arg_v[0];
            const right = arg_v[1];

            const s_v = left.split("<:>");
            const atom = atom_method[s_v[0]];
            if (atom) {
                let arg_v;
                if (s_v.length === 2) {
                    arg_v = JSON.parse(s_v[1]);
                    arg_v.push(right);
                } else {
                    arg_v = [right];
                }

                if (atom.len === arg_v.length) {
                    return await atom.fn(arg_v);
                } else {
                    return `${s_v[0]}<:>${JSON.stringify(arg_v)}`;
                }
            }

            throw new Error("what fuck");
            // return left;
        },
        len: 2
    }
};

const self = {
    __init: async () => {
        await config.__init();
    },
    __release: async () => {
        await config.__release();
    },
    match: async (left, right) => {
        return atom_method.match.fn([left, right]);
    }
};

export default self;

