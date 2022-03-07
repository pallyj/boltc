; ModuleID = 'test2'
source_filename = "test2"

%"2L5S5test2Int64" = type { i64 }

define %"2L5S5test2Int64" @"3L5S5F3test2Int64max"() {
max:
  ret %"2L5S5test2Int64" { i64 9223372036854775807 }
}

define %"2L5S5test2Int64" @"3L5S5F3test2Int64min"() {
min:
  ret %"2L5S5test2Int64" { i64 -9223372036854775808 }
}

declare void @printi(%"2L5S5test2Int64")

declare void @printiraw(i64)

define i64 @"2L5F9test2factorial"(i64 %0) {
factorial:
  %cmp_gt = icmp ugt i64 %0, 1
  br i1 %cmp_gt, label %then, label %continue

then:                                             ; preds = %factorial
  %diff = sub i64 %0, 1
  %call = call i64 @"2L5F9test2factorial"(i64 %diff)
  %mul = mul i64 %0, %call
  %init = insertvalue %"2L5S5test2Int64" undef, i64 %mul, 0
  call void @printi(%"2L5S5test2Int64" %init)
  ret i64 %mul
  br label %continue

continue:                                         ; preds = %then, %factorial
  ret i64 1
}

define i64 @"2L5F4test2main"() {
main:
  %call = call %"2L5S5test2Int64" @"3L5S5F3test2Int64max"()
  %member_access = extractvalue %"2L5S5test2Int64" %call, 0
  call void @printiraw(i64 %member_access)
  ret i64 0
}
