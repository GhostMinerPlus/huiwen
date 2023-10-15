import node_rsa from 'node-rsa';

import config from "./config.js";

const atom_method = {
    insert: {
        fn: async arg_v => {
            const fn = arg_v[0];
            const id = arg_v[1];
            const val = arg_v[2];

            await surreal.query(
                `insert into ${fn} { id: $id, val: $val } on duplicate key update val = $val`,
                { id, val }
            );
            return fn;
        },
        len: 3
    },
    delete: {
        fn: async arg_v => {
            const fn = arg_v[0];
            const id = arg_v[1];

            await surreal.query(
                `delete ${fn} where meta::id(id) = $id`,
                { id }
            );
            return id;
        },
        len: 2
    },
    remove: {
        fn: async arg_v => {
            const fn = arg_v[0];

            await surreal.query(
                `remove table ${fn}`
            );
            return fn;
        },
        len: 1
    },
    watch: {
        fn: async arg_v => {
            const id = arg_v[0];
            if (id === "fn") {
                let rs = await surreal.query(
                    "info for db"
                );
                const fn_mp = rs[0].result.tables;
                const r = {}
                for (const fn in fn_mp) {
                    r[fn] = fn;
                }
                for (const fn in atom_method) {
                    r[fn] = fn;
                }
                return r;
            }

            const atom = atom_method[arg_v[0]];
            if (atom) {
                return arg_v[0];
            }

            const rs = await surreal.query(
                `select meta::id(id) as id, val from ${arg_v[0]}`
            );
            const r = {};
            for (const route of rs[0].result) {
                r[route.id] = route.val;
            }
            return r;
        },
        len: 1
    },
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

            let rs = await surreal.query(
                `select value val from ${left} where meta::id(id) = $id`,
                { id: right }
            );

            if (rs[0].result[0]) {
                left = rs[0].result[0];
            } else {
                rs = await surreal.query(
                    `select value val from ${left} where meta::id(id) = "?"`
                );
                left = rs[0].result[0] ? rs[0].result[0] : "";
            }
            return left;
        },
        len: 2
    },
    encrypt: {
        fn: async arg_v => {
            const data = arg_v[0];
            const encrypter = new node_rsa(config.public_key);
            return encrypter.encrypt(data, 'base64');
        },
        len: 1
    },
    decrypt: {
        fn: async arg_v => {
            const data = arg_v[0];
            const decrypter = new node_rsa(config.private_key);
            return decrypter.decrypt(data, 'utf8');
        },
        len: 1
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

