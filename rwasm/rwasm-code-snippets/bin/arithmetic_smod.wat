(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 0)
    (local i32 i64 i32 i64 i64 i64 i64 i64 i64 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 0
    global.set 0
    i32.const 32768
    i32.const 0
    i64.load offset=32768
    local.tee 1
    i32.wrap_i64
    local.tee 2
    i32.sub
    i64.load align=1
    local.set 3
    i32.const 32776
    local.get 2
    i32.sub
    i64.load align=1
    local.set 4
    i32.const 32784
    local.get 2
    i32.sub
    i64.load align=1
    local.set 5
    i32.const 32792
    local.get 2
    i32.sub
    i64.load align=1
    local.set 6
    i32.const 0
    local.get 1
    i64.const 32
    i64.shl
    local.tee 7
    i64.const -137438953472
    i64.add
    i64.const 32
    i64.shr_s
    local.tee 8
    i64.store offset=32768
    i32.const 32768
    local.get 8
    i32.wrap_i64
    local.tee 2
    i32.sub
    local.tee 9
    i64.load align=1
    local.set 10
    i32.const 32776
    local.get 2
    i32.sub
    i64.load align=1
    local.set 1
    i32.const 32784
    local.get 2
    i32.sub
    i64.load align=1
    local.set 11
    i32.const 32792
    local.get 2
    i32.sub
    i64.load align=1
    local.set 12
    i32.const 0
    local.get 7
    i64.const -274877906944
    i64.add
    i64.const 32
    i64.shr_s
    i64.store offset=32768
    local.get 4
    i64.const 56
    i64.shl
    local.get 4
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 4
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 4
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 4
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 4
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 4
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 4
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    local.set 13
    local.get 1
    i64.const 56
    i64.shl
    local.get 1
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 1
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 1
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 1
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 1
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 1
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 1
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    local.set 14
    block  ;; label = @1
      block  ;; label = @2
        local.get 10
        local.get 3
        i64.ne
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        i64.ne
        br_if 0 (;@2;)
        local.get 14
        local.set 13
        local.get 11
        local.get 5
        i64.ne
        br_if 0 (;@2;)
        i64.const 0
        local.set 4
        local.get 14
        local.set 13
        i64.const 0
        local.set 7
        i64.const 0
        local.set 15
        i64.const 0
        local.set 16
        local.get 12
        local.get 6
        i64.eq
        br_if 1 (;@1;)
      end
      i64.const 0
      local.set 4
      block  ;; label = @2
        local.get 3
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        local.get 13
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        local.get 5
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        i64.const 0
        local.set 7
        i64.const 0
        local.set 15
        i64.const 0
        local.set 16
        local.get 6
        i64.const 72057594037927936
        i64.eq
        br_if 1 (;@1;)
      end
      local.get 6
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 6
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 6
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 6
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      local.set 16
      local.get 6
      i64.const 56
      i64.shl
      local.get 6
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.get 6
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 6
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.set 17
      local.get 5
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 5
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 5
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 5
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      local.set 18
      local.get 5
      i64.const 56
      i64.shl
      local.get 5
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.get 5
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 5
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.set 19
      local.get 3
      i64.const 56
      i64.shl
      local.get 3
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.get 3
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 3
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.get 3
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 3
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 3
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 3
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      i64.or
      local.set 4
      local.get 12
      i64.const 56
      i64.shl
      local.get 12
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.get 12
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 12
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.get 12
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 12
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 12
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 12
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      i64.or
      local.set 15
      local.get 11
      i64.const 56
      i64.shl
      local.get 11
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.get 11
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 11
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.get 11
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 11
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 11
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 11
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      i64.or
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          local.get 10
          i64.const 56
          i64.shl
          local.get 10
          i64.const 65280
          i64.and
          i64.const 40
          i64.shl
          i64.or
          local.get 10
          i64.const 16711680
          i64.and
          i64.const 24
          i64.shl
          local.get 10
          i64.const 4278190080
          i64.and
          i64.const 8
          i64.shl
          i64.or
          i64.or
          local.get 10
          i64.const 8
          i64.shr_u
          i64.const 4278190080
          i64.and
          local.get 10
          i64.const 24
          i64.shr_u
          i64.const 16711680
          i64.and
          i64.or
          local.get 10
          i64.const 40
          i64.shr_u
          i64.const 65280
          i64.and
          local.get 10
          i64.const 56
          i64.shr_u
          i64.or
          i64.or
          i64.or
          local.tee 20
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          local.get 20
          local.set 3
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 12
            i64.eqz
            br_if 0 (;@4;)
            i64.const 0
            local.get 15
            i64.sub
            local.set 15
            local.get 20
            local.set 1
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 11
              i64.eqz
              br_if 0 (;@5;)
              local.get 7
              i64.const -1
              i64.add
              local.set 7
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 1
              i64.eqz
              br_if 0 (;@5;)
              i64.const -1
              local.set 7
              local.get 14
              i64.const -1
              i64.add
              local.set 14
              br 1 (;@4;)
            end
            i64.const -1
            local.set 7
            local.get 20
            i64.const -1
            i64.add
            local.set 1
            i64.const 0
            local.set 15
            i64.const -1
            local.set 14
            br 1 (;@3;)
          end
          i64.const 0
          local.set 15
          local.get 20
          local.set 1
        end
        local.get 7
        i64.const -1
        i64.xor
        local.set 7
        local.get 14
        i64.const -1
        i64.xor
        local.set 14
        local.get 1
        i64.const -1
        i64.xor
        local.set 3
      end
      local.get 17
      local.get 16
      i64.or
      local.set 16
      local.get 19
      local.get 18
      i64.or
      local.set 1
      block  ;; label = @2
        local.get 4
        i64.const -1
        i64.gt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            i64.eqz
            br_if 0 (;@4;)
            i64.const 0
            local.get 16
            i64.sub
            local.set 16
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 5
            i64.eqz
            br_if 0 (;@4;)
            local.get 1
            i64.const -1
            i64.add
            local.set 1
            i64.const 0
            local.set 16
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 13
            i64.eqz
            br_if 0 (;@4;)
            i64.const -1
            local.set 1
            local.get 13
            i64.const -1
            i64.add
            local.set 13
            i64.const 0
            local.set 16
            br 1 (;@3;)
          end
          i64.const -1
          local.set 1
          local.get 4
          i64.const -1
          i64.add
          local.set 4
          i64.const 0
          local.set 16
          i64.const -1
          local.set 13
        end
        local.get 1
        i64.const -1
        i64.xor
        local.set 1
        local.get 13
        i64.const -1
        i64.xor
        local.set 13
        local.get 4
        i64.const -1
        i64.xor
        local.set 4
      end
      local.get 0
      local.get 3
      i64.const 56
      i64.shl
      local.get 3
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 21
      local.get 3
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 3
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 5
      local.get 3
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 3
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 3
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 3
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      local.tee 22
      i64.or
      local.tee 6
      i64.const 8
      i64.shr_u
      i32.wrap_i64
      local.tee 23
      i32.store8 offset=1
      local.get 0
      local.get 6
      i64.const 16
      i64.shr_u
      i32.wrap_i64
      local.tee 24
      i32.store8 offset=2
      local.get 0
      local.get 4
      i64.const 56
      i64.shr_u
      local.tee 11
      i64.store8 offset=32
      local.get 0
      local.get 4
      i64.const 56
      i64.shl
      local.get 4
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 25
      local.get 4
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 4
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 10
      local.get 4
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 4
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 4
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 11
      i64.or
      i64.or
      local.tee 26
      i64.or
      local.tee 11
      i64.const 8
      i64.shr_u
      local.tee 27
      i64.store8 offset=33
      local.get 0
      local.get 11
      i64.const 16
      i64.shr_u
      local.tee 28
      i64.store8 offset=34
      local.get 0
      local.get 14
      i64.const 56
      i64.shr_u
      local.tee 17
      i64.store8 offset=8
      local.get 0
      local.get 14
      i64.const 56
      i64.shl
      local.get 14
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 29
      local.get 14
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 14
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 12
      local.get 14
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 14
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 14
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 17
      i64.or
      i64.or
      local.tee 30
      i64.or
      local.tee 17
      i64.const 8
      i64.shr_u
      local.tee 31
      i64.store8 offset=9
      local.get 0
      local.get 13
      i64.const 56
      i64.shr_u
      local.tee 19
      i64.store8 offset=40
      local.get 0
      local.get 13
      i64.const 56
      i64.shl
      local.get 13
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 32
      local.get 13
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 13
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 18
      local.get 13
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 13
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 13
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 19
      i64.or
      i64.or
      local.tee 33
      i64.or
      local.tee 19
      i64.const 8
      i64.shr_u
      local.tee 34
      i64.store8 offset=41
      local.get 0
      local.get 7
      i64.const 56
      i64.shr_u
      local.tee 35
      i64.store8 offset=16
      local.get 0
      local.get 7
      i64.const 56
      i64.shl
      local.get 7
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 36
      local.get 7
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 7
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 37
      local.get 7
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 7
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 7
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 35
      i64.or
      i64.or
      local.tee 38
      i64.or
      local.tee 35
      i64.const 8
      i64.shr_u
      local.tee 39
      i64.store8 offset=17
      local.get 0
      local.get 1
      i64.const 56
      i64.shr_u
      local.tee 40
      i64.store8 offset=48
      local.get 0
      local.get 1
      i64.const 56
      i64.shl
      local.get 1
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 41
      local.get 1
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 1
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 42
      local.get 1
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 1
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 1
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 40
      i64.or
      i64.or
      local.tee 43
      i64.or
      local.tee 40
      i64.const 8
      i64.shr_u
      local.tee 44
      i64.store8 offset=49
      local.get 0
      local.get 15
      i64.const 56
      i64.shr_u
      local.tee 45
      i64.store8 offset=24
      local.get 0
      local.get 15
      i64.const 56
      i64.shl
      local.get 15
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 46
      local.get 15
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 15
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 47
      local.get 15
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 15
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 15
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 45
      i64.or
      i64.or
      local.tee 48
      i64.or
      local.tee 45
      i64.const 8
      i64.shr_u
      local.tee 49
      i64.store8 offset=25
      local.get 0
      local.get 16
      i64.const 56
      i64.shr_u
      local.tee 50
      i64.store8 offset=56
      local.get 0
      local.get 16
      i64.const 56
      i64.shl
      local.get 16
      i64.const 65280
      i64.and
      i64.const 40
      i64.shl
      i64.or
      local.tee 51
      local.get 16
      i64.const 16711680
      i64.and
      i64.const 24
      i64.shl
      local.get 16
      i64.const 4278190080
      i64.and
      i64.const 8
      i64.shl
      i64.or
      i64.or
      local.tee 52
      local.get 16
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 16
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 16
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 50
      i64.or
      i64.or
      local.tee 53
      i64.or
      local.tee 50
      i64.const 8
      i64.shr_u
      local.tee 54
      i64.store8 offset=57
      local.get 0
      local.get 22
      i32.wrap_i64
      local.tee 2
      i32.store8
      local.get 0
      local.get 35
      i64.const 16
      i64.shr_u
      i32.wrap_i64
      local.tee 55
      i32.store8 offset=18
      local.get 0
      local.get 40
      i64.const 16
      i64.shr_u
      i32.wrap_i64
      local.tee 56
      i32.store8 offset=50
      local.get 0
      local.get 45
      i64.const 16
      i64.shr_u
      i32.wrap_i64
      local.tee 57
      i32.store8 offset=26
      local.get 0
      local.get 50
      i64.const 16
      i64.shr_u
      i32.wrap_i64
      local.tee 58
      i32.store8 offset=58
      local.get 0
      local.get 6
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 59
      i32.store8 offset=3
      local.get 0
      local.get 11
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 60
      i32.store8 offset=35
      local.get 0
      local.get 17
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 61
      i32.store8 offset=11
      local.get 0
      local.get 19
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 62
      i32.store8 offset=43
      local.get 0
      local.get 35
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 63
      i32.store8 offset=19
      local.get 0
      local.get 40
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 64
      i32.store8 offset=51
      local.get 0
      local.get 45
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 65
      i32.store8 offset=27
      local.get 0
      local.get 50
      i64.const 24
      i64.shr_u
      i32.wrap_i64
      local.tee 66
      i32.store8 offset=59
      local.get 0
      local.get 5
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 67
      i32.store8 offset=4
      local.get 0
      local.get 10
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 68
      i32.store8 offset=36
      local.get 0
      local.get 12
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 69
      i32.store8 offset=12
      local.get 0
      local.get 18
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 70
      i32.store8 offset=44
      local.get 0
      local.get 37
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 71
      i32.store8 offset=20
      local.get 0
      local.get 17
      i64.const 16
      i64.shr_u
      local.tee 6
      i64.store8 offset=10
      local.get 0
      local.get 19
      i64.const 16
      i64.shr_u
      local.tee 11
      i64.store8 offset=42
      local.get 0
      local.get 47
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 72
      i32.store8 offset=28
      local.get 0
      local.get 42
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 73
      i32.store8 offset=52
      local.get 0
      local.get 52
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 74
      i32.store8 offset=60
      local.get 0
      local.get 5
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 75
      i32.store8 offset=5
      local.get 0
      local.get 10
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 76
      i32.store8 offset=37
      local.get 0
      local.get 12
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 77
      i32.store8 offset=13
      local.get 0
      local.get 18
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 78
      i32.store8 offset=45
      local.get 0
      local.get 37
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 79
      i32.store8 offset=21
      local.get 0
      local.get 42
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 80
      i32.store8 offset=53
      local.get 0
      local.get 47
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 81
      i32.store8 offset=29
      local.get 0
      local.get 52
      i64.const 40
      i64.shr_u
      i32.wrap_i64
      local.tee 82
      i32.store8 offset=61
      local.get 0
      local.get 21
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 83
      i32.store8 offset=6
      local.get 0
      local.get 25
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 84
      i32.store8 offset=38
      local.get 0
      local.get 29
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 85
      i32.store8 offset=14
      local.get 0
      local.get 32
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 86
      i32.store8 offset=46
      local.get 0
      local.get 36
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 87
      i32.store8 offset=22
      local.get 0
      local.get 41
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 88
      i32.store8 offset=54
      local.get 0
      local.get 46
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 89
      i32.store8 offset=30
      local.get 0
      local.get 51
      i64.const 48
      i64.shr_u
      i32.wrap_i64
      local.tee 90
      i32.store8 offset=62
      local.get 0
      local.get 3
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 91
      i32.store8 offset=7
      local.get 0
      local.get 4
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 92
      i32.store8 offset=39
      local.get 0
      local.get 14
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 93
      i32.store8 offset=15
      local.get 0
      local.get 13
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 94
      i32.store8 offset=47
      local.get 0
      local.get 7
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 95
      i32.store8 offset=23
      local.get 0
      local.get 1
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 96
      i32.store8 offset=55
      local.get 0
      local.get 15
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 97
      i32.store8 offset=31
      local.get 0
      local.get 16
      i64.const 255
      i64.and
      i32.wrap_i64
      local.tee 98
      i32.store8 offset=63
      local.get 26
      i32.wrap_i64
      local.set 99
      i32.const 0
      local.set 100
      i32.const 0
      local.set 101
      block  ;; label = @2
        local.get 2
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 101
        local.get 23
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 2
        local.set 101
        local.get 24
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 3
        local.set 101
        local.get 59
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 4
        local.set 101
        local.get 67
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 5
        local.set 101
        local.get 75
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 6
        local.set 101
        local.get 83
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 7
        local.set 101
        local.get 91
        br_if 0 (;@2;)
        i32.const 8
        local.set 101
        local.get 30
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 9
        local.set 101
        local.get 31
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 10
        local.set 101
        local.get 6
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 11
        local.set 101
        local.get 61
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 12
        local.set 101
        local.get 69
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 13
        local.set 101
        local.get 77
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 14
        local.set 101
        local.get 85
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 15
        local.set 101
        local.get 93
        br_if 0 (;@2;)
        i32.const 16
        local.set 101
        local.get 38
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 17
        local.set 101
        local.get 39
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 18
        local.set 101
        local.get 55
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 19
        local.set 101
        local.get 63
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 20
        local.set 101
        local.get 71
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 21
        local.set 101
        local.get 79
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 22
        local.set 101
        local.get 87
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 23
        local.set 101
        local.get 95
        br_if 0 (;@2;)
        i32.const 24
        local.set 101
        local.get 48
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 25
        local.set 101
        local.get 49
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 26
        local.set 101
        local.get 57
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 27
        local.set 101
        local.get 65
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 28
        local.set 101
        local.get 72
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 29
        local.set 101
        local.get 81
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 30
        local.set 101
        local.get 89
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 31
        i32.const 0
        local.get 97
        select
        local.set 101
      end
      block  ;; label = @2
        local.get 99
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 100
        local.get 27
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 2
        local.set 100
        local.get 28
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 3
        local.set 100
        local.get 60
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 4
        local.set 100
        local.get 68
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 5
        local.set 100
        local.get 76
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 6
        local.set 100
        local.get 84
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 7
        local.set 100
        local.get 92
        br_if 0 (;@2;)
        i32.const 8
        local.set 100
        local.get 33
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 9
        local.set 100
        local.get 34
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 10
        local.set 100
        local.get 11
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 11
        local.set 100
        local.get 62
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 12
        local.set 100
        local.get 70
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 13
        local.set 100
        local.get 78
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 14
        local.set 100
        local.get 86
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 15
        local.set 100
        local.get 94
        br_if 0 (;@2;)
        i32.const 16
        local.set 100
        local.get 43
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 17
        local.set 100
        local.get 44
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 18
        local.set 100
        local.get 56
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 19
        local.set 100
        local.get 64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 20
        local.set 100
        local.get 73
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 21
        local.set 100
        local.get 80
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 22
        local.set 100
        local.get 88
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 23
        local.set 100
        local.get 96
        br_if 0 (;@2;)
        i32.const 24
        local.set 100
        local.get 53
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 25
        local.set 100
        local.get 54
        i32.wrap_i64
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 26
        local.set 100
        local.get 58
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 27
        local.set 100
        local.get 66
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 28
        local.set 100
        local.get 74
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 29
        local.set 100
        local.get 82
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 30
        local.set 100
        local.get 90
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 31
        i32.const 0
        local.get 98
        select
        local.set 100
      end
      local.get 100
      i32.const 31
      i32.xor
      local.set 62
      i32.const 32
      local.get 100
      i32.sub
      local.set 92
      local.get 100
      local.get 0
      i32.const 72
      i32.add
      i32.add
      i32.const -24
      i32.add
      local.set 61
      local.get 101
      i32.const 32
      i32.or
      local.get 100
      i32.sub
      local.set 2
      local.get 0
      i32.const -1
      i32.add
      local.set 70
      local.get 0
      i32.const 32
      i32.add
      i32.const 32
      i32.add
      local.set 69
      local.get 0
      i32.const 32
      i32.add
      local.get 100
      i32.add
      local.set 76
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          local.get 101
          i32.add
          local.set 59
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  local.tee 84
                  local.get 101
                  i32.sub
                  local.tee 23
                  local.get 92
                  i32.or
                  i32.const 8
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 92
                  local.get 23
                  i32.gt_u
                  br_if 3 (;@4;)
                  local.get 84
                  local.get 101
                  i32.eq
                  br_if 5 (;@2;)
                  local.get 92
                  local.get 23
                  i32.sub
                  local.set 60
                  block  ;; label = @8
                    local.get 23
                    i32.const -1
                    i32.add
                    i32.const -1
                    i32.gt_s
                    br_if 0 (;@8;)
                    local.get 69
                    local.get 23
                    i32.sub
                    local.set 24
                    i32.const 0
                    local.set 91
                    br 2 (;@6;)
                  end
                  local.get 70
                  local.get 101
                  i32.add
                  local.set 75
                  local.get 69
                  local.get 23
                  i32.sub
                  local.set 83
                  i32.const 0
                  local.set 91
                  loop  ;; label = @8
                    i32.const 0
                    local.set 2
                    block  ;; label = @9
                      loop  ;; label = @10
                        local.get 2
                        i32.const 1
                        i32.add
                        local.set 100
                        local.get 59
                        local.get 2
                        i32.add
                        i32.load8_u
                        local.set 99
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 60
                            local.get 2
                            i32.add
                            i32.const 0
                            i32.lt_s
                            br_if 0 (;@12;)
                            local.get 99
                            i32.const 255
                            i32.and
                            local.tee 99
                            local.get 83
                            local.get 2
                            i32.add
                            i32.load8_u
                            local.tee 2
                            i32.gt_u
                            br_if 3 (;@9;)
                            local.get 99
                            local.get 2
                            i32.lt_u
                            br_if 7 (;@5;)
                            local.get 100
                            local.get 23
                            i32.lt_u
                            br_if 1 (;@11;)
                            br 3 (;@9;)
                          end
                          local.get 99
                          i32.const 255
                          i32.and
                          br_if 2 (;@9;)
                          local.get 100
                          local.get 23
                          i32.ge_u
                          br_if 2 (;@9;)
                        end
                        local.get 100
                        local.set 2
                        br 0 (;@10;)
                      end
                    end
                    i32.const 0
                    local.set 24
                    local.get 23
                    local.set 100
                    local.get 62
                    local.set 99
                    i32.const 0
                    local.set 2
                    block  ;; label = @9
                      loop  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 99
                            i32.const 0
                            i32.lt_s
                            br_if 0 (;@12;)
                            local.get 75
                            local.get 100
                            i32.add
                            local.tee 67
                            local.get 24
                            local.get 76
                            local.get 99
                            i32.add
                            i32.load8_u
                            local.tee 68
                            i32.sub
                            local.get 67
                            i32.load8_u
                            local.tee 24
                            i32.add
                            i32.store8
                            local.get 68
                            local.get 2
                            i32.const 255
                            i32.and
                            i32.add
                            local.get 24
                            i32.gt_u
                            local.set 2
                            local.get 99
                            i32.const -1
                            i32.add
                            local.set 99
                            br 1 (;@11;)
                          end
                          local.get 2
                          i32.const 255
                          i32.and
                          i32.eqz
                          br_if 2 (;@9;)
                          local.get 75
                          local.get 100
                          i32.add
                          local.tee 2
                          local.get 2
                          i32.load8_u
                          local.tee 2
                          i32.const -1
                          i32.add
                          i32.store8
                          local.get 2
                          i32.eqz
                          local.set 2
                        end
                        i32.const 0
                        local.get 2
                        i32.sub
                        local.set 24
                        local.get 100
                        i32.const -1
                        i32.add
                        local.tee 100
                        i32.const 0
                        i32.gt_s
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 91
                    i32.const 1
                    i32.add
                    local.set 91
                    br 0 (;@8;)
                  end
                end
                local.get 0
                i64.const 0
                i64.store offset=64
                i32.const 0
                local.set 91
                i32.const 0
                local.set 2
                i32.const 0
                local.set 100
                i32.const 0
                local.set 99
                i32.const 0
                local.set 24
                i32.const 0
                local.set 60
                i32.const 0
                local.set 67
                i32.const 0
                local.set 68
                i32.const 0
                local.set 75
                block  ;; label = @7
                  local.get 84
                  local.get 101
                  i32.eq
                  local.tee 83
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 64
                  i32.add
                  local.get 23
                  i32.sub
                  i32.const 8
                  i32.add
                  local.get 59
                  local.get 23
                  call 2
                  drop
                  local.get 0
                  i32.load8_u offset=71
                  local.set 2
                  local.get 0
                  i32.load8_u offset=70
                  local.set 100
                  local.get 0
                  i32.load8_u offset=69
                  local.set 99
                  local.get 0
                  i32.load8_u offset=68
                  local.set 24
                  local.get 0
                  i32.load8_u offset=67
                  local.set 60
                  local.get 0
                  i32.load8_u offset=66
                  local.set 67
                  local.get 0
                  i32.load8_u offset=65
                  local.set 68
                  local.get 0
                  i32.load8_u offset=64
                  local.set 75
                end
                local.get 0
                i64.const 0
                i64.store offset=72
                local.get 61
                local.get 76
                local.get 92
                call 2
                drop
                local.get 75
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.const 16
                i64.shl
                local.get 68
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.const 8
                i64.shl
                i64.or
                local.get 67
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.or
                i64.const 32
                i64.shl
                local.get 60
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.const 24
                i64.shl
                local.get 24
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.const 16
                i64.shl
                i64.or
                i64.or
                local.get 99
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.const 8
                i64.shl
                local.get 100
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.or
                i64.or
                i64.const 8
                i64.shl
                local.get 2
                i64.extend_i32_u
                i64.const 255
                i64.and
                i64.or
                local.set 1
                block  ;; label = @7
                  local.get 0
                  i64.load offset=72
                  local.tee 4
                  i64.const 56
                  i64.shl
                  local.get 4
                  i64.const 65280
                  i64.and
                  i64.const 40
                  i64.shl
                  i64.or
                  local.get 4
                  i64.const 16711680
                  i64.and
                  i64.const 24
                  i64.shl
                  local.get 4
                  i64.const 4278190080
                  i64.and
                  i64.const 8
                  i64.shl
                  i64.or
                  i64.or
                  local.get 4
                  i64.const 8
                  i64.shr_u
                  i64.const 4278190080
                  i64.and
                  local.get 4
                  i64.const 24
                  i64.shr_u
                  i64.const 16711680
                  i64.and
                  i64.or
                  local.get 4
                  i64.const 40
                  i64.shr_u
                  i64.const 65280
                  i64.and
                  local.get 4
                  i64.const 56
                  i64.shr_u
                  i64.or
                  i64.or
                  i64.or
                  local.tee 4
                  i64.const 0
                  i64.eq
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 1
                  local.get 4
                  i64.div_u
                  local.tee 7
                  i64.const 255
                  i64.and
                  local.get 4
                  i64.mul
                  i64.sub
                  local.set 1
                  local.get 7
                  i32.wrap_i64
                  local.set 91
                end
                local.get 0
                local.get 1
                i64.const 56
                i64.shl
                local.get 1
                i64.const 65280
                i64.and
                i64.const 40
                i64.shl
                i64.or
                local.get 1
                i64.const 16711680
                i64.and
                i64.const 24
                i64.shl
                local.get 1
                i64.const 4278190080
                i64.and
                i64.const 8
                i64.shl
                i64.or
                i64.or
                local.get 1
                i64.const 8
                i64.shr_u
                i64.const 4278190080
                i64.and
                local.get 1
                i64.const 24
                i64.shr_u
                i64.const 16711680
                i64.and
                i64.or
                local.get 1
                i64.const 40
                i64.shr_u
                i64.const 65280
                i64.and
                local.get 1
                i64.const 56
                i64.shr_u
                i64.or
                i64.or
                i64.or
                i64.store offset=64
                local.get 83
                br_if 1 (;@5;)
                local.get 59
                local.get 0
                i32.const 64
                i32.add
                local.get 23
                i32.sub
                i32.const 8
                i32.add
                local.get 23
                call 2
                drop
                br 1 (;@5;)
              end
              loop  ;; label = @6
                i32.const 0
                local.set 2
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 2
                    i32.const 1
                    i32.add
                    local.set 100
                    local.get 59
                    local.get 2
                    i32.add
                    i32.load8_u
                    local.set 99
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 60
                        local.get 2
                        i32.add
                        i32.const 0
                        i32.lt_s
                        br_if 0 (;@10;)
                        local.get 99
                        i32.const 255
                        i32.and
                        local.tee 99
                        local.get 24
                        local.get 2
                        i32.add
                        i32.load8_u
                        local.tee 2
                        i32.gt_u
                        br_if 3 (;@7;)
                        local.get 99
                        local.get 2
                        i32.lt_u
                        br_if 5 (;@5;)
                        local.get 100
                        local.get 23
                        i32.ge_u
                        br_if 3 (;@7;)
                        br 1 (;@9;)
                      end
                      local.get 99
                      i32.const 255
                      i32.and
                      br_if 2 (;@7;)
                      local.get 100
                      local.get 23
                      i32.ge_u
                      br_if 2 (;@7;)
                    end
                    local.get 100
                    local.set 2
                    br 0 (;@8;)
                  end
                end
                local.get 91
                i32.const 1
                i32.add
                local.set 91
                br 0 (;@6;)
              end
            end
            local.get 101
            i32.const 31
            i32.gt_u
            br_if 0 (;@4;)
            local.get 101
            local.set 2
            local.get 91
            i32.const 255
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              loop  ;; label = @6
                local.get 0
                local.get 2
                i32.add
                i32.load8_u
                br_if 1 (;@5;)
                i32.const 32
                local.set 101
                local.get 2
                i32.const 1
                i32.add
                local.tee 2
                i32.const 32
                i32.eq
                br_if 2 (;@4;)
                br 0 (;@6;)
              end
            end
            local.get 2
            local.set 101
          end
          local.get 84
          i32.const 1
          i32.add
          local.set 2
          local.get 84
          i32.const 31
          i32.le_u
          br_if 0 (;@3;)
        end
        block  ;; label = @3
          local.get 20
          i64.const -1
          i64.gt_s
          br_if 0 (;@3;)
          local.get 0
          i32.const 0
          local.get 0
          i32.load8_u offset=31
          local.tee 2
          i32.sub
          i32.store8 offset=31
          local.get 0
          local.get 0
          i32.load8_u offset=30
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 2
          select
          i32.store8 offset=30
          local.get 0
          local.get 0
          i32.load8_u offset=29
          local.tee 99
          i32.const -1
          i32.xor
          i32.const 0
          local.get 99
          i32.sub
          local.get 2
          local.get 100
          i32.or
          local.tee 100
          i32.const 255
          i32.and
          select
          i32.store8 offset=29
          local.get 0
          local.get 0
          i32.load8_u offset=28
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 100
          local.get 99
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=28
          local.get 0
          local.get 0
          i32.load8_u offset=27
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=27
          local.get 0
          local.get 0
          i32.load8_u offset=26
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=26
          local.get 0
          local.get 0
          i32.load8_u offset=25
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=25
          local.get 0
          local.get 0
          i32.load8_u offset=24
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=24
          local.get 0
          local.get 0
          i32.load8_u offset=23
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=23
          local.get 0
          local.get 0
          i32.load8_u offset=22
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=22
          local.get 0
          local.get 0
          i32.load8_u offset=21
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=21
          local.get 0
          local.get 0
          i32.load8_u offset=20
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=20
          local.get 0
          local.get 0
          i32.load8_u offset=19
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=19
          local.get 0
          local.get 0
          i32.load8_u offset=18
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=18
          local.get 0
          local.get 0
          i32.load8_u offset=17
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=17
          local.get 0
          local.get 0
          i32.load8_u offset=16
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=16
          local.get 0
          local.get 0
          i32.load8_u offset=15
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=15
          local.get 0
          local.get 0
          i32.load8_u offset=14
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=14
          local.get 0
          local.get 0
          i32.load8_u offset=13
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=13
          local.get 0
          local.get 0
          i32.load8_u offset=12
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=12
          local.get 0
          local.get 0
          i32.load8_u offset=11
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=11
          local.get 0
          local.get 0
          i32.load8_u offset=10
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=10
          local.get 0
          local.get 0
          i32.load8_u offset=9
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=9
          local.get 0
          local.get 0
          i32.load8_u offset=8
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=8
          local.get 0
          local.get 0
          i32.load8_u offset=7
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=7
          local.get 0
          local.get 0
          i32.load8_u offset=6
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=6
          local.get 0
          local.get 0
          i32.load8_u offset=5
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=5
          local.get 0
          local.get 0
          i32.load8_u offset=4
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=4
          local.get 0
          local.get 0
          i32.load8_u offset=3
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=3
          local.get 0
          local.get 0
          i32.load8_u offset=2
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 0
          local.get 2
          i32.sub
          local.get 99
          local.get 100
          i32.or
          local.tee 99
          i32.const 255
          i32.and
          select
          i32.store8 offset=2
          local.get 0
          local.get 0
          i32.load8_u offset=1
          local.tee 100
          i32.const -1
          i32.xor
          i32.const 0
          local.get 100
          i32.sub
          local.get 99
          local.get 2
          i32.or
          local.tee 2
          i32.const 255
          i32.and
          select
          i32.store8 offset=1
          local.get 0
          local.get 0
          i32.load8_u
          local.tee 99
          i32.const -1
          i32.xor
          i32.const 0
          local.get 99
          i32.sub
          local.get 2
          local.get 100
          i32.or
          i32.const 255
          i32.and
          select
          i32.store8
        end
        local.get 0
        i64.load offset=24
        local.tee 1
        i64.const 56
        i64.shl
        local.get 1
        i64.const 65280
        i64.and
        i64.const 40
        i64.shl
        i64.or
        local.get 1
        i64.const 16711680
        i64.and
        i64.const 24
        i64.shl
        local.get 1
        i64.const 4278190080
        i64.and
        i64.const 8
        i64.shl
        i64.or
        i64.or
        local.get 1
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get 1
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get 1
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get 1
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        local.set 16
        local.get 0
        i64.load offset=16
        local.tee 1
        i64.const 56
        i64.shl
        local.get 1
        i64.const 65280
        i64.and
        i64.const 40
        i64.shl
        i64.or
        local.get 1
        i64.const 16711680
        i64.and
        i64.const 24
        i64.shl
        local.get 1
        i64.const 4278190080
        i64.and
        i64.const 8
        i64.shl
        i64.or
        i64.or
        local.get 1
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get 1
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get 1
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get 1
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        local.set 15
        local.get 0
        i64.load offset=8
        local.tee 1
        i64.const 56
        i64.shl
        local.get 1
        i64.const 65280
        i64.and
        i64.const 40
        i64.shl
        i64.or
        local.get 1
        i64.const 16711680
        i64.and
        i64.const 24
        i64.shl
        local.get 1
        i64.const 4278190080
        i64.and
        i64.const 8
        i64.shl
        i64.or
        i64.or
        local.get 1
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get 1
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get 1
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get 1
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        local.set 7
        local.get 0
        i64.load
        local.tee 1
        i64.const 56
        i64.shl
        local.get 1
        i64.const 65280
        i64.and
        i64.const 40
        i64.shl
        i64.or
        local.get 1
        i64.const 16711680
        i64.and
        i64.const 24
        i64.shl
        local.get 1
        i64.const 4278190080
        i64.and
        i64.const 8
        i64.shl
        i64.or
        i64.or
        local.get 1
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get 1
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get 1
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get 1
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        local.set 4
        br 1 (;@1;)
      end
      loop  ;; label = @2
        br 0 (;@2;)
      end
    end
    i32.const 0
    local.get 8
    i64.store offset=32768
    local.get 9
    local.get 16
    i64.const 56
    i64.shl
    local.get 16
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 16
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 16
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 16
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 16
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 16
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 16
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store offset=24 align=1
    local.get 9
    local.get 15
    i64.const 56
    i64.shl
    local.get 15
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 15
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 15
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 15
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 15
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 15
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 15
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store offset=16 align=1
    local.get 9
    local.get 7
    i64.const 56
    i64.shl
    local.get 7
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 7
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 7
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 7
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 7
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 7
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 7
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store offset=8 align=1
    local.get 9
    local.get 4
    i64.const 56
    i64.shl
    local.get 4
    i64.const 65280
    i64.and
    i64.const 40
    i64.shl
    i64.or
    local.get 4
    i64.const 16711680
    i64.and
    i64.const 24
    i64.shl
    local.get 4
    i64.const 4278190080
    i64.and
    i64.const 8
    i64.shl
    i64.or
    i64.or
    local.get 4
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 4
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 4
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 4
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store align=1
    local.get 0
    i32.const 80
    i32.add
    global.set 0)
  (func (;1;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
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
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
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
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
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
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
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
  (func (;2;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 1)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "arithmetic_smod" (func 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
