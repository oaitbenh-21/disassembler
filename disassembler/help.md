# **format**
- |magic>name>padding>size>description>padding>instruction|
- **magic** => 4byte
- **name** => 128byte
- **padding** => 4byte
- **description** => 2048byte
- **size** => 2byte
- **description** => 2048byte
# **pcode**
- 01 => Register
- 10 => Direct
- 11 => Indirect
- 00 => nothing not found if there is arguments
# **has index**
- just for Direct
- if has index true Direct == 2bytes else 4bytes
