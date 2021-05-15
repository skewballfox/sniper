// /"use strict";
const {promisify}=require('util');
const SniperClient= require('../index.node');
//const connectAsync=promisify(connectSniper);


//const { Sniper } = addon;
function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
      currentDate = Date.now();
    } while (currentDate - date < milliseconds);
  }



SniperClient.init();
sleep(2000)
SniperClient.add_target("12345","test.py","python");
sleep(2000)

//console.log();
// const { promisify } = require("util");

// const { connectSniper } = require("../index.node");

// const connectAsync = promisify(connectSniper);

// Example
// (async () => {
//     const node = await connectAsync();
    
//     console.log(node);
// })();
// console.log(typeof (node));
// *


// Example
// (async () => {
//     const node = await connectAsync();
    
//     console.log(node);
// })();
//console.log(x.get('language'));
//console.log(process.version);

//export default addon;

