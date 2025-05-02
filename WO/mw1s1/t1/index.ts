//  Running teh learning tests here

// Adding color chalk
import chalk from "chalk";

// Writing the object

const engine = {
  working: true,
};

/* 
The intention bug in both the objects is tha its sharing the engine
property so, if u change the engine property in one object
it will change the engine property in other object too
- So thats why when calling the checkEngine function
- The engine property for both is false
*/

const mustang = {
  name: "Mustang",
  engine: engine, // <-- This is the problem same property in the second
};

const camaro = {
  name: "Camaro",
  engine: engine, // <-- Same issue here
};

// Since this one function is called on both objects
// Engine status become false
function checkEngine(car: { name: string; engine: { working: boolean } }) {
  if (car.name === "Mustang") {
    car.engine.working = false;
  }
}

checkEngine(mustang);
checkEngine(camaro);

console.log(chalk.green(JSON.stringify(mustang, null, 2)));
console.log(chalk.yellow(JSON.stringify(camaro, null, 2)));
