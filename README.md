# 백준 풀기

## 문제 새로 풀 때

```bash
$ python3 problem.py [problem_num] && cargo fmt
```

## 각 문제 테스팅

아직도 debugger를 잘 못 쓰기 때문에.. `println!`을 쓰고 싶을 때를 대비하여 항상 `--nocapture` 플래그 켜기

```bash
$ cargo test [problem_num] -- --nocapture
```

## TODO?

한 백 문제 넘어가면 너무 복잡해질 것 같아서

`src/problems/1000-2000/p01008.rs` 이런 식으로 생성해볼지 생각중
