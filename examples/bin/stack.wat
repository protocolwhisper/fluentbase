(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func))
  (import "env" "_sys_halt" (func $_sys_halt (type 0)))
  (func $__get_stack_pointer (type 1) (result i32)
    global.get $__stack_pointer)
  (func $deploy (type 2))
  (func $main (type 2)
    call $__get_stack_pointer
    call $_sys_halt)
  (memory (;0;) 2)
  (global $__stack_pointer (mut i32) (i32.const 66560))
  (export "memory" (memory 0))
  (export "deploy" (func $deploy))
  (export "main" (func $main)))