# SparkContext 클래스 정의

## Reference
https://github.com/apache/spark/blob/master/python/pyspark/context.py
실제 spark 코드를 참고해 단순하게 필요한 부분만 갖고 와 정의했다.

## SparkContext란?
Main entry point for Spark functionality이자 Connection to a Spark cluster (그외 RDD 생성, 변수 브로드캐스트 등) 즉 **스파크의 핵심 중추**.

## SparkContext 의 기능
~~#### 1) App Name 정의, 변경~~
#### 2) executor에게 application code, task 를 전송/전달해 job을 시킨다.
#### 3) Cluster Manager로부터 자원을 할당 받음


## Fields to include
~~- appName(str) : A name for your job, to display on the cluster web UI.~~
- pyFiles(list) : Collection of .py files to send to the cluster and add to PYTHONPATH.
- master(str or int) :  Cluster URL to connect to (e.g. mesos://host:port, spark://host:port, local[4]). 여기선 예시처럼 복잡한 걸 받지 않고, app name 처럼 고유한 이름 혹은 숫자를 받는 것으로 하자.<br>


## Methods to include
### 1) SparkContext 객체 생성과 해제
- getOrCreate: Get or instantiate a :class:`SparkContext` and register it as a singleton object. **간단하게 싱글톤을 구현해보자(Rust official tutorial 17.3장)**
- stop: Shut down the `SparkContext`. 여기서는 포인터 등 할당된 자원을 해제하는 관점으로 접근해 구현해보자.
<br>

### 2) job으로 실행할 .py 파일 리스트에 파일 추가, 전체 리스트 반환
- addPyFile(self, path: str): Add a .py dependency for all tasks to be executed on this SparkContext in the future.
- listFiles(self): Returns a list of file paths that are added to resources.
<br>

### 3) Executor 그리고 Cluster Manager와 엮이는 부분 (1,2 먼저 하고 나중에)
- runJob
- resources: Cluster Manager 로부터 자원 할당 받음
<br>

### 4) 아주 나중에 구현할 수도 있는 것
- defaultMinPartitions
- emptyRDD: 원래 SparkContext는 RDD를 생성해주는 기능이 있다. 여기서는 일단 RDD 클래스를 안 만들 예정이지만, 생각 바뀌면 넣는 걸로 한다.
<br><br>


## 학습 목표
- 3~6, 9장 (어디에나 들어갈 기본 사항들)
- 8장: file list 필드를 통해 이를 다뤄보자.
- 15장: job 할당/해제 등에 사용될 여지가 있지 않을까 하는 idea
- 17장: 가장 먼저 구현하는 클래스가 이 SparkContext이기 때문이다.