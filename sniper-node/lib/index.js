// /"use strict";
import addon, { startSniper } from '../index.node';

//const { Sniper } = addon;
class Sniper {
    constructor(config) {
        this.boxed = startSniper(config);
    }
}
let x = new Sniper("snippets");

console.log(x)
console.log(typeof (x));
//console.log(x.get('language'));
//console.log(process.version);

export default addon;

