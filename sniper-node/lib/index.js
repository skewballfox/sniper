// /"use strict";
const {promisify}=require('util');
const addon= require('../index.node');
const get_triggers=promisify(addon.get_triggers);
const get_snippet=promisify(addon.get_snippet);

module.exports.get_triggers=get_triggers
module.exports.get_snippet=get_snippet
module.exports.drop_target=addon.drop_target
module.exports.add_target=addon.add_target


//const { Sniper } = addon;



/*
class Sniper{
  constructor(){
    this.boxed=SniperClient.init()
  }
  add_target(session_id,uri,language){
    SniperClient.add_target(this.boxed,session_id,uri,language);
  }
}
let x=new Sniper();
console.log(x);
x.add_target("23456","test.py","python");
*/

//SniperClient.add_target("23456","test.py","python");


// Example
// (async () => {
//     let trigs = await get_triggers("23456","test.py");
    
//     //trigs.forEach(function(entry) {
//     //  console.log(entry);
//     //});
//     console.log(trigs)
// })();
// (async () => {
//   let snippet = await get_snippet("python","if");
  
  
//     console.log(snippet);
  
// })();

//SniperClient.drop_target("12345","test.py","python");
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

