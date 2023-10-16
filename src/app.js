import api from "../api/index.js";
import engine from "moon";

const root = document.getElementById("root");

const self = {
    __init: async () => {
        await engine.__init(api);
        root.innerText = "操你慧群";
    }
};

export default self;