; ModuleID = 'test'
source_filename = "test"

define i64 @factorial(i64 %0) {
factorial:
  %cmp_eq = icmp eq i64 %0, 0
  br i1 %cmp_eq, label %then, label %finally

then:                                             ; preds = %factorial
  ret i64 1
  br label %continue

finally:                                          ; preds = %factorial
  %diff = sub i64 %0, 1
  %call = call i64 @factorial(i64 %diff)
  %mul = mul i64 %0, %call
  ret i64 %mul

continue:                                         ; preds = %then
  ret i64 0
}
