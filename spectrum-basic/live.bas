10 randomize 1
20 let maxX=6: rem 32
30 let maxY=6: rem 22
40 dim a$(2, maxY, maxX)
50 let s$ = "*" : rem solid square is shift 8 in graphics mode
60 let curr = 1

70 go sub 1500

80 print at 0,0; a$(curr, 1)
90 for y=2 to maxY
100 print a$(curr, y)
110 next y

120 for y=1 to maxY
130 for x=1 to maxX

140 let prevX = x - 1
150 if prevX < 1 then let prevX = maxX
160 let prevY = y - 1
170 if prevY < 1 then let prevY = maxY
180 let nextX = x + 1
190 if nextX > maxX then let nextX = 1
200 let nextY = y + 1
210 if nextY > maxY then let nextY = 1

220 let n = 0
230 if code a$(curr,prevY,x) - 32 then let n = n + 1
240 if code a$(curr,prevY,nextX) - 32 then let n = n + 1
250 if code a$(curr,y,nextX) - 32 then let n = n + 1
260 if code a$(curr,nextY,nextX) - 32 then let n = n + 1
270 if code a$(curr,nextY,x) - 32 then let n = n + 1
280 if code a$(curr,nextY,prevX) - 32 then let n = n + 1
290 if code a$(curr,y,prevX) - 32 then let n = n + 1
300 if code a$(curr,prevY,prevX) - 32 then let n = n + 1

310 let alive = 0
320 if code a$(curr,y,x) - 32 then go to 350
330 rem live cell check
340 if n = 2 or n = 3 then let alive = 1 : go to 370
350 rem dead cell check
360 if n = 3 then let alive = 1

370 let n$ = " "
380 if alive then let n$ = s$
390 let a$(3 - curr, y, x) = n$

400 next x
410 next y

420 let curr = 3 - curr
430 go to 80

1000 rem random init
1010 for y = 1 to maxY
1020 for x = 1 to maxX
1030 let randBool = int (rnd*2)
1040 if randBool = 1 then let a$(1,y,x) = s$
1050 next x
1060 next y
1070 return

1500 rem glider init
1510 let a$(1,2,4) = s$
1520 let a$(1,3,2) = s$
1530 let a$(1,3,4) = s$
1540 let a$(1,4,3) = s$
1550 let a$(1,4,4) = s$
1560 return
