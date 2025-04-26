(module
  (import "env" "imported_func" (func $imported_func (param i32) (result i32)))
  (import "env" "imported_global" (global $imported_global (mut i32)))
  (import "env" "imported_memory" (memory $imported_memory 1))
  (import "env" "imported_table" (table $imported_table 10 funcref))
  (memory $exported_memory 2)
  (table $exported_table 5 funcref)
  (global $exported_global_i32 (mut i32) (i32.const 42))
  (global $exported_global_f64 (mut f64) (f64.const 3.14159))
  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $subtract (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $use_imports (param i32) (result i32)
    global.get $imported_global
    local.get 0
    i32.add
    call $imported_func
  )
  (func $table_func (param i32) (result i32)
    local.get 0
    i32.const 10
    i32.mul
  )
  (elem (table $exported_table) (i32.const 0) $table_func)
  (export "add" (func $add))
  (export "subtract" (func $subtract))
  (export "use_imports" (func $use_imports))
  (export "answer" (global $exported_global_i32))
  (export "pi" (global $exported_global_f64))
  (export "mem" (memory $exported_memory))
  (export "tbl" (table $exported_table))
)