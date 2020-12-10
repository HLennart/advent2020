use std::thread::current;

fn main() {
    let input = INPUT
        .split("\n")
        .map(|input| input.parse::<usize>().unwrap());

    let mut current_jolts: usize = 0;

    let mut number_dif_1 = 0;
    // 1 because we count the inbuilt adapter
    let mut number_dif_3 = 1;

    let mut adapter_list = input.clone().collect::<Vec<_>>();
    let length = adapter_list.len();
    for _ in 0..length {
        let smallest_dif_element = (1..4)
            .map(|i| {
                (
                    i,
                    adapter_list
                        .iter()
                        .filter(move |element| current_jolts + i == **element)
                        .nth(0),
                )
            })
            .filter(|x| x.1.is_some())
            .min_by(|x, y| x.0.cmp(&y.0))
            .unwrap();
        println!("{:?}", smallest_dif_element);
        if smallest_dif_element.0 == 1 {
            number_dif_1 += 1;
        } else if smallest_dif_element.0 == 3 {
            number_dif_3 += 1;
        }
        // panic if this doesnt work
        let element_position = adapter_list
            .iter()
            .position(|x| *x == *smallest_dif_element.1.unwrap())
            .unwrap();
        current_jolts += smallest_dif_element.0;

        adapter_list.remove(element_position);
    }

    println!("{}", number_dif_1 * number_dif_3);
}

const CHARGING_OUTLET: usize = 0;

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
