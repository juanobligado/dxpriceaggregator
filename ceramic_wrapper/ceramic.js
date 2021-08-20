const ceramic = require('ceramic') /* the current working directory so that means main.js because of package.json */
//let theFile = process.argv /* what the user enters as first argument */

console.log(
    ceramic(process.argv)
)