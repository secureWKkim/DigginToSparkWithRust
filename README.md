# Diggin' to Spark with Rust!
This repository includes example codes and learning resources for 사내 Data Team Rust 지식 공유 미팅.<br><br>

## Contents
Spark Architecture를 매우 간단한 수준에서 추상화 해본다.<br>

원하는 기능을 구현하기 위해 필요한 문법이 무엇인지, 빌드/실행 시 어떤 컴파일/런타임 에러를 만나게 되는지 직접 부딪혀 보며 배우는 내용을 담고 있다.<br>

단, Rust 공식 문서 속 (거의) 모든 키워드들이 한 번 이상 포함되도록 한다.
<br><br>

### 주제 선정 이유?
- Spark 구조와 동작 방식 concept을 느낌으로써 분산 처리에 대한 학습까지 꾀하기 위함
- dask, k8s, airflow 등 타 DE 관련 기술에서도 spark의 자원 관리 방식, DAG 스케줄링 원리와 비슷한 concept/pattern이 종종 발견되기 때문
- 본인이 과거에 spark를 나름 많이 공부했기 때문
- Hadoop&Spark에도 JVM 즉 GC가 있음 <-> Rust는 없음. 하다못해 그 유명한 Go마저 GC가 있다! 때문에 Rust가 대용량 트래픽 서버 및 빅데이터 개발자와 친해질 언어라는 사실을 아는 사람은 다 알고, 발빠르게 미래를 준비하고 있음 (그래서인지 몰라도 요새 데이터 관련 오픈소스들에 rust 많이 보임). 이 부분에서 영감 받음.
<br><br>

## Timeline
- 23.08.21 Mon: oop, test, derive, mod, impl, trait
<br><br><br>

### Mostly Used Commands
It works on 'diggin_to_spark_with_rust' folder.<br><br>

MacOS/Linux
- cargo build
- ./target/debug/diggin_to_spark_with_rust
- cargo run
- cargo check
<br>

Window
- cargo build
- ./target/debug/diggin_to_spark_with_rust.exe
- cargo run
- cargo check

### Reference
https://thewayaboutme.tistory.com/543
https://github.com/apache/spark/tree/master/python/pyspark
