(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func))
  (import "env" "_sys_read" (func $_sys_read (type 0)))
  (import "env" "_ecc_secp256k1_verify" (func $_ecc_secp256k1_verify (type 1)))
  (import "env" "_sys_write" (func $_sys_write (type 2)))
  (import "env" "_sys_halt" (func $_sys_halt (type 3)))
  (func $deploy (type 4))
  (func $main (type 4)
    (local i32)
    global.get $__stack_pointer
    i32.const 160
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 24
    i32.add
    i64.const 0
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i64.const 0
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i64.const 0
    i64.store
    local.get 0
    i64.const 0
    i64.store
    local.get 0
    i32.const 0
    i32.const 32
    call $_sys_read
    drop
    local.get 0
    i32.const 38
    i32.add
    i32.const 0
    i32.const 64
    call $memset
    drop
    local.get 0
    i32.const 38
    i32.add
    i32.const 32
    i32.const 64
    call $_sys_read
    drop
    local.get 0
    i32.const 0
    i32.store8 offset=102
    local.get 0
    i32.const 102
    i32.add
    i32.const 96
    i32.const 1
    call $_sys_read
    drop
    local.get 0
    i32.const 103
    i32.add
    i32.const 0
    i32.const 33
    call $memset
    drop
    local.get 0
    i32.const 103
    i32.add
    i32.const 97
    i32.const 33
    call $_sys_read
    drop
    block  ;; label = @1
      local.get 0
      i32.const 32
      local.get 0
      i32.const 38
      i32.add
      i32.const 64
      local.get 0
      i32.const 103
      i32.add
      i32.const 33
      local.get 0
      i32.load8_u offset=102
      call $_ecc_secp256k1_verify
      br_if 0 (;@1;)
      local.get 0
      i32.const 148
      i32.add
      i64.const 0
      i64.store align=4
      local.get 0
      i32.const 1
      i32.store offset=140
      local.get 0
      i32.const 1048596
      i32.store offset=136
      local.get 0
      i32.const 1048604
      i32.store offset=144
      local.get 0
      i32.const 136
      i32.add
      call $_ZN4core9panicking9panic_fmt17h78607b33a29a727dE
      unreachable
    end
    local.get 0
    i32.const 160
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core9panicking9panic_fmt17h78607b33a29a727dE (type 3) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 1048604
    call $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hed637ffe26dba6a3E
    block  ;; label = @1
      local.get 1
      i64.load
      i64.const -4493808902380553279
      i64.xor
      local.get 1
      i32.const 8
      i32.add
      i64.load
      i64.const -163230743173927068
      i64.xor
      i64.or
      i64.const 0
      i64.ne
      br_if 0 (;@1;)
      local.get 1
      local.get 1
      call $_sys_write
    end
    i32.const -71
    call $_sys_halt
    loop  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hed637ffe26dba6a3E (type 2) (param i32 i32)
    local.get 0
    i64.const 568815540544143123
    i64.store offset=8
    local.get 0
    i64.const 5657071353825360256
    i64.store)
  (func $_ZN17compiler_builtins3mem6memset17h7e84e2271aaccac9E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 4
      i32.const -4
      i32.and
      local.tee 2
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 2
        loop  ;; label = @3
          local.get 5
          local.get 2
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 4
      i32.const 3
      i32.and
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memset (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memset17h7e84e2271aaccac9E)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048604))
  (global (;2;) i32 (i32.const 1048608))
  (export "memory" (memory 0))
  (export "deploy" (func $deploy))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (i32.const 1048576) "verification failed\00\00\00\10\00\13\00\00\00"))
