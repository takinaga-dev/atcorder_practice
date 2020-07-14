process.stdin.resume();
process.stdin.setEncoding('utf8');

var input_lines = [];
var reader = require('readline').createInterface({
  input: process.stdin,
  output: process.stdout
});

reader.on('line', (line) => {
  input_lines.push(line)
})

reader.on('close', () => {
  value = parseInt(input_lines[0])
  while (value - 1000 > 0) {
    value = value - 1000;
  }
  console.log(1000 - value)
})