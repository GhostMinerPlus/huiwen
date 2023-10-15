import api from "../api/index.js";
import engine from "moon";

const self = {
    __init: async () => {
        await engine.__init(api);

        const root = document.getElementById("root");

        root.innerText = await engine.execute(["add", "1", "1"]);
    }
};

export default self;