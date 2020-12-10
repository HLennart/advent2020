use std::collections::HashMap;
use std::num::Wrapping;

fn main() {
    let input = INPUT
        .split("\n")
        .map(|input| Wrapping(input.parse::<usize>().unwrap()));

    let mut possible_ways = HashMap::new();
    possible_ways.insert(0, 1usize);

    let mut adapter_list = input.clone().collect::<Vec<_>>();
    adapter_list.sort();

    for adapter_jolt in adapter_list {
        let mut count: usize = 0;
        for i in 1..4 {
            if let Some(val) = possible_ways.get(&((adapter_jolt - Wrapping(i)).0)) {
                count += val
            }
        }
        if let Some(val) = possible_ways.get_mut(&adapter_jolt.0) {
            *val += count;
        } else {
            possible_ways.insert(adapter_jolt.0, count);
        }
    }

    let largest = possible_ways.keys().into_iter().max().unwrap();
    println!("{:?}", possible_ways.get(largest));
}

const TEST_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

const INPUT: &str = "149
87
67
45
76
29
107
88
4
11
118
160
20
115
130
91
144
152
33
94
53
148
138
47
104
121
112
116
99
105
34
14
44
137
52
2
65
141
140
86
84
81
124
62
15
68
147
27
106
28
69
163
97
111
162
17
159
122
156
127
46
35
128
123
48
38
129
161
3
24
60
58
155
22
55
75
16
8
78
134
30
61
72
54
41
1
59
101
10
85
139
9
98
21
108
117
131
66
23
77
7
100
51";
