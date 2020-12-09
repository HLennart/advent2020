fn main() {
    let input = INPUT
        .split("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let result = input
        .iter()
        .enumerate()
        .skip(PREAMBLE_LENGTH)
        .map(|(index, s)| {
            let preamble_length_before = &input[index - PREAMBLE_LENGTH..index];
            let mut is_valid = false;
            for (i, val) in preamble_length_before.iter().enumerate() {
                for (j, j_val) in preamble_length_before.iter().enumerate() {
                    if val + j_val == *s && i != j {
                        is_valid = true;
                    }
                }
            }

            println!("{} is valid?: {}", s, is_valid);
            println!(
                "preamble_length before: {:?}",
                &input[index - PREAMBLE_LENGTH..index]
            );
            (s, is_valid)
        });

    println!(
        "{:?}",
        result.filter(|(_, is_valid)| *is_valid == false).nth(0)
    );
}

const PREAMBLE_LENGTH: usize = 25;

const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

const INPUT: &str = "12
26
18
15
16
21
42
47
39
35
41
24
36
20
43
50
38
5
34
44
2
45
25
17
11
22
59
52
7
9
12
16
72
32
28
48
13
47
27
19
14
20
18
37
15
21
26
31
23
33
24
86
51
65
55
46
30
79
29
56
28
32
34
38
112
35
48
36
39
41
44
47
66
52
53
59
84
64
86
58
69
57
60
61
62
113
92
70
71
93
74
75
97
112
91
123
99
105
109
110
115
117
146
196
132
182
118
179
131
133
309
149
221
200
272
165
172
190
201
204
222
455
437
219
387
232
378
249
349
380
308
480
386
331
354
498
394
337
355
362
612
391
412
405
577
441
451
468
481
540
557
989
586
749
639
961
686
668
685
691
912
746
692
796
753
803
863
817
1149
892
1090
919
949
1172
1499
1371
1666
1324
1482
1414
1481
1353
1359
1376
1383
1438
1495
1445
1549
2358
2272
1709
1736
1811
2039
1868
2091
2308
2496
2677
2683
2707
2712
2729
2735
2821
2736
3631
2759
2828
2883
2940
2994
3959
5340
3547
3445
3827
4974
3907
4575
5533
4804
5173
5360
5390
5419
6181
6273
5471
5580
8334
5587
5877
6439
8470
5934
10384
8966
9032
7272
14422
7734
16261
9297
9977
10592
17031
10533
12115
11521
10890
18926
23470
11051
23533
22092
11811
15174
13206
14966
17455
15006
16238
25766
18162
22740
17711
26017
38132
22572
26064
21423
43995
21941
29232
24257
29977
28762
42419
25017
29522
34400
29444
28172
29972
44179
33949
52544
35873
39134
39652
40283
73701
52016
44513
43364
45680
67896
46198
71536
49274
53189
116120
84814
69174
68455
57616
58144
62121
63921
69822
73083
126071
75007
78786
83647
84796
96529
105485
133095
102463
129845
99387
95472
106890
107418
110805
127966
115760
119737
120265
145768
131227
126042
232932
203419
153793
162433
163582
180176
192001
194859
211232
197935
201850
202362
219652
202890
281759
214308
218223
265505
235497
295936
240002
257269
271810
279835
289624
316226
317375
326015
469745
343758
372177
569459
405252
413082
421502
404212
416670
417198
421113
458225
483728
475499
492766
497271
511812
519837
561434
551645
885685
891247
633601
905848
743213
715935
968843
838700
809464
1403119
817294
1397497
820882
833868
909964
932925
1240484
959227
968265
1400134
1465536
1927492
2143347
1725115
1185246
1349536
1376814
2749670
1621783
1459148
1730846
1626758
1630346
1780109
1638176
1651162
3540844
1654750
1901190
2807029
2308763
3363291
2606441
2153511
2534782
3031564
2823422
3097324
2562060
2644394
2726350
2835962
3110310
3080931
3264934
5370744
5369089
7905526
3289338
5179176
4759952
5413470
8402308
4463250
5358204
5385482
4715571
5141223
4879861
5096842
5206454
6345865
5288410
5946272
5562312
5807281
5916893
6191241
8466413
6554272
10676845
8468514
8851650
7752588
9178821
9223202
19900047
9343111
9560092
9595432
11192763
9812413
9976703
10168271
10385252
11205303
10850722
12108134
11369593
11479205
11724174
12471165
12745513
15020685
14306860
16221102
16604238
17095699
16931409
22856417
19980684
18903203
18938543
21039297
29349751
24997388
19789116
24469687
20553523
21235974
35346157
22220315
22848798
23093767
23203379
25216678
26778025
37457076
33245403
30527962
32825340
33535647
41025090
35834612
41592820
37841746
52474190
44439353
40342639
59037991
43756902
45705661
41789497
43402321
45423694
58194955
62673754
55674138
56448782
48420057
55744640
57305987
63353302
66360987
64063609
68659952
69370259
78184385
73676358
122391293
79631243
84099541
88196255
82132136
85191818
119027440
87213191
97534137
99851103
106615012
104094195
104164697
104868839
105726044
143046617
185730392
90433990
127416911
142247994
132723561
155808494
149001502
172405009
153307601
179482346
187064294
166231677
167323954
169345327
210709207
177647181
191307386
217850901
190285093
196160034
194528185
257757944
223157551
232681984
280724512
256665667
239435492
260140472
274971555
381683486
340308888
302309103
319539278
320631555
354388248
362391711
446950760
642617991
450425565
384813278
451447858
381592479
390688219
386445127
640170833
480915495
604841037
455839535
472117476
496101159
641823958
499575964
721922315
770987136
621848381
622940658
705444833
673927526
777133346
753079930
1226826671
766405757
1163578473
836870692
2003960017
768037606
772280698
842284662
858562603
951940694
1395221356
927957011
955415499
968218635
995677123
2622048027
1122516622
1244789039
1295775907
1328385491
1296868184
1633401844
1427007456
1611642533
1935859171
1626600209
1538686455
1540318304
1604908298
2296604126
1610322268
1981079225
1700847265
2538279279
1907356193
1883372510
1896175646
2622277332
1963895758
2118193745
2367305661
2450902113
2900684205
2592644091
2907190452
2901776482
3231508507
2965693911
3079004759
3143594753
3907623965
3215230566
3803531839
3305755563
6809400447
4905584940
3584219775
3597022911
4502175037
5491575968
4001566255
4014369391
4082089503
6537264070
5083887656
5807874657
5043546204
6908756707
5494420573
5808966934
5867470393
6294235325
6449350316
7491843740
6358825319
6520986129
8800176136
6889975338
6902778474
7666309278
9508789964
7679112414
7598589166
8015935646
10308604716
9810533189
8096458894
9576510076
12653060644
10127433860
10537966777
11303387507
11853245892
13248800657
14310170971
12161705718
12743585641
12879811448
20740644397
13261603793
13410961467
13792753812
14488564504
17440745251
21077270745
17187902378
15277701580
15614524812
16112394540
17672968970
26510404450
18223892754
19703943936
20665400637
25096141319
22391212669
24733057340
25572667185
24905291359
34533398209
25041517166
25623397089
26672565260
32016646566
29407278624
27203715279
28281318316
44684980204
30892226392
43129184113
31726919352
33287493782
33838417566
33785363510
42578260329
37927836690
38889293391
45398457977
43056613306
54140335964
47124270009
56464893577
69729178566
61134197976
52245232445
51714082426
52295962349
53876280539
55485033595
83574273595
58930634631
62619145744
84768506931
64179720174
65014413134
72674656901
71215330472
67623781076
71713200200
83326294667
97819928022
108760855926
88455071283
94770695732
98838352435
148340707801
103959314871
104010044775
104541194794
166483895932
105590362965
109361314134
112806915170
118104179339
132638194210
123110354805
147506014841
131803501250
129194133308
136727613334
139336981276
155039494867
150950075743
160168271483
261147622971
183225767015
187293423718
270443210803
193609048167
228032485743
208500509665
236344696044
208551239569
217348109964
214951677099
348648543034
268531114584
301329946354
379251802310
377516381447
252304488113
260997634558
276064594610
280144209051
315207766350
388200757226
305989570610
353777319650
432299787063
370519190733
376834815182
380902471885
408560725266
402109557832
488649184157
417051749234
423502916668
628792752085
591272360960
467256165212
768586111566
513302122671
1058528526172
528369082723
755886877482
586133779661
537062229168
556208803661
840554665902
621197336960
659766890260
930478640555
821033484862
1428353001826
747354005915
1179389794150
797954221119
810670283098
819161307066
884307914446
945420831957
1618987705981
980558287883
995625247935
1004318394380
1280964227220
1396763469563
1065431311891
1941046079892
2175707007741
1093271032829
1158259566128
1177406140621
2562243416852
1631661920361
1407120896175
1640194791928
1545308227034
2586510690325
2722714367655
1703469221512
1608624504217
1976183535818
2374235260049
1829728746403
1925979119840
1984876682263
1999943642315
2181724535001
4289979911837
2335665706749
2817600932549
3339984101129
2584527036796
2500391929004
3343664013440
3236849642578
4331338871872
3679652757330
4722658009970
2952429123209
3153932731251
3248777448546
3312093725729
4648693487495
5134153658210
3438353250620
3755707866243
3814605428666
8087046738115
3910855802103
4517390241750
4181668177316
4682116464005
6778337351749
6157585033678
5084918965800
7004485314789
5536956160005
6655757739169
6106361854460
6189278765787
9595938284298
6201206571755
6264522848938
7930893912551
8331995670416
6560871174275
6750446976349
7194061116863
7252958679286
11274197731587
7725461230769
7996273605982
10839701497683
10068440835781
9767035429805
8863784641321
10219072624010
13944508093212
10621875125805
11191280820260
11643318014465
11726234925792
15785217050085
20058230596208
12390485337542
12465729420693
12762077746030
13458583965801
14892866844691
18951356511817
14557144780257
14447019796149
15190334722845
14978419910055
15721734836751
16589245872090
16860058247303
18630820071126
28437003875856
28232563886555
31146390652347
20840947749815
21813155946065
22265193140270
42872625578139
23369552940257
24116720263334
34203641283607
25849069303343
25152563083572
36562682586566
26220661711831
27905603761950
29004164576406
34673091348568
32581793084054
43415423785911
44725899413157
49218622243600
32310980708841
45993510833387
35490878318429
39471767820941
55263110915681
68721694881482
42654103695880
43106140890085
45929876209399
50337381975165
47486273203591
66785434367661
49269283346906
55224826288237
51373224795403
70163969666997
77593886507234
54126265473781
75687933974139
61315145285247
90655775622556";
