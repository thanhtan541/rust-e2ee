import e2ee_sdk

do {
    let _ = try add(a: 18446744073709551615, b: 1)
    fatalError("Should have thrown a IntegerOverflow exception!")
} catch ArithmeticError.IntegerOverflow {
    // It's okay!
}

assert(try! add(a: 2, b: 4) == 6, "add work")
assert(try! add(a: 4, b: 8) == 12, "add work")

