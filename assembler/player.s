.name "brahem"
.comment "the ultimate warrior"
    live %1
.macro ldstore
    ld %34, r2
    st r2, 45
.endmacro
    live %1
.usemacro ldstore
