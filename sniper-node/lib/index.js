// /"use strict";
import addon, { startSniper } from '../index.node';

//const { Sniper } = addon;
class Sniper {
    constructor() {
        this.session = startSniper();
    }
    //TODO: define api
}
let x = new Sniper();

console.log(x)
console.log(typeof (x));
//console.log(x.get('language'));
//console.log(process.version);

export default addon;

