# ft_linear_regression

Rust로 구현한 단순 선형 회귀 프로그램입니다.

자동차의 주행거리(`km`)를 입력으로 받아 가격(`price`)을 예측하며, 경사하강법으로 `theta_0`, `theta_1`을 학습합니다.

## 구성

- `train`: 데이터셋을 읽어 모델을 학습합니다.
- `predict`: 주행거리를 입력받아 가격을 예측합니다.
- `rmse.png`: epoch별 RMSE 그래프
- `scatter_line.png`: 데이터 산점도와 추세선 그래프
- `params_YYYYMMDD_HHMMSS.yaml`: 학습된 모델 파라미터

## 데이터 형식

CSV 헤더는 아래 형식을 기대합니다.

```csv
km,price
```

## 로컬 실행

빌드:

```bash
cargo build --release
```

정규화 없이 학습:

```bash
./target/release/train --input ./data/data.csv --epoch 1000
```

정규화해서 학습:

```bash
./target/release/train --input ./data/data.csv --epoch 1000 --alpha 0.01 --normalize
```

예측:

```bash
./target/release/predict
```

실행 후 아래처럼 주행거리를 입력합니다.

```text
Enter mileage:
```

## Docker 실행

이미지 빌드:

```bash
./docker-build.sh
```

학습:

```bash
./docker-train.sh --input ./data/data.csv --epoch 1000 --alpha 0.01 --normalize
```

예측:

```bash
./docker-predict.sh
```

위 스크립트들은 현재 디렉터리를 컨테이너에 마운트하므로, 학습 후 생성된 YAML과 그래프 파일이 로컬 디렉터리에 그대로 남습니다.

## 참고

- 학습 시 `--normalize` 옵션을 사용하면 주행거리(`km`)에 대해 z-score 정규화를 적용합니다.
- 정규화된 모델은 `x_mean`, `x_std`를 함께 저장하며, 예측 시 동일한 정규화를 다시 적용합니다.
- YAML 파일이 없으면 `theta_0 = 0`, `theta_1 = 0`으로 예측합니다.
