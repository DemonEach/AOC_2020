const fs = require('fs');

const REQUIRED_FIELDS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'];

function readMyFile(file_name) {
    return fs.readFileSync(file_name, 'utf-8').split(/\n\s*\n/);
}

function isValid(_line) {
    for (const val of REQUIRED_FIELDS) {
        if (_line.indexOf(val) < 0) {
            return false;
        }
    }
    return true;
}

function solution(passports) {
    let count = 0;
    
    for (const passport of passports) {
        if (isValid(passport)) {
            count++;
        }
    }

    return count;
}

var splited_passwords = readMyFile('./sample-01.txt');

console.log(solution(splited_passwords));
