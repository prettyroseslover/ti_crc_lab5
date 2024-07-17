# Лабораторная работа. Теория информации 

> Реализация CRC и последующее его использование для подписи сообщения по схеме ЭльГамаля.
 
```
$ cargo run

Output

CRC for message 1000 is 11010 with the generator polynomial, v15: 101001
Let's take a look at the collision statistics!
01000 was there 8 times
11011 was there 8 times
10110 was there 8 times
11000 was there 8 times
00100 was there 8 times
01011 was there 8 times
10100 was there 8 times
00110 was there 8 times
11100 was there 8 times
11111 was there 8 times
10011 was there 8 times
00010 was there 8 times
01110 was there 8 times
10001 was there 8 times
00101 was there 8 times
01111 was there 8 times
10000 was there 8 times
01001 was there 8 times
00001 was there 8 times
10010 was there 8 times
00111 was there 8 times
10111 was there 8 times
11001 was there 8 times
01100 was there 8 times
11110 was there 8 times
01010 was there 8 times
01101 was there 8 times
10101 was there 8 times
11101 was there 8 times
11010 was there 8 times
00000 was there 8 times
00011 was there 8 times
Now it's time to sign the same message 8 using ElGamal signature scheme
p, g, x for v15 are 163, 7, 5
Open key y = 18
r = 7^5 mod 163 = 18
u = ( 26 - 5 * 18 ) mod 162 = 98
s = 65 * 98 mod 162 = 52
Signed (M, r, s): (8, 18, 52)
Is the message 8 signed with r = 18? true
Is the message 8 signed with r = 17? false
```
