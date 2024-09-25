# flanning ✈️

<p align="center">
  <img src="https://i.imgur.com/9zkv8nA.png" width="800">
</p>

플래닝은 **여행의 목적**과 **계획 스타일**이 각기 다른 여행자들이 자신에게 적합한 서비스를 이용할 수 있도록 돕는 **국내 여행 공동 플래닝 서비스**입니다.

## 📈 Background

일상으로의 완전한 전환이 이루어진 현재, **국내 여행 수요**는 2022년부터 코로나 19 이전 수준을 거의 회복했습니다. 엔데믹 이후 증가한 여행자 수와 함께 **국내 여행 서비스**에 대한 수요도 증가하고 있습니다.  
플래닝은 **각기 다른 여행 스타일**을 가진 여행자들이 **자율적**으로 계획을 세울 수 있는, **개인화된 여행 경험**을 제공하는 것을 목표로 합니다.

## 👩‍💻 Members

- 신선림- Project Manager
- 조호진- Product Designer
- 임희원- Frontend, Backend
- 정재윤- Backend

## 🛠️Features

<p align="center">
  <img src="https://i.imgur.com/uH0Y36M.png" width="800">
  <img src="https://i.imgur.com/Ku8oG7q.png" width="800">
  <img src="https://i.imgur.com/175MpS4.png" width="800">
  <img src="https://i.imgur.com/9rDGkXi.png" width="800">
  <img src="https://i.imgur.com/Mr0nZCP.png" width="800">
  <img src="https://i.imgur.com/mu0UUaF.png" width="800">
</p>

### 📍 로그인 (로그인 유지)

- 회원가입 후, 로그인 시 앱을 킬 때 **자동 로그인** 기능 제공.
- **카카오 계정 연동**으로 회원가입 후, 여행 취향 설정 페이지로 자동 이동.

### 📍 홈 (푸시 알림)

- **푸시 알림 설정**: 친구가 자신을 **추가**하거나, **여행에 초대**했을 때 푸시 알림 수신.
- **친구 초대**: 카카오톡이나 링크를 통해 자신의 친구 코드를 공유해 친구를 초대할 수 있습니다.

### 📍 일정 (일정 작성 및 관리)

- **일정 작성**: 지도 검색창을 통해 위치를 입력하고 일정을 생성. 하루 이상의 일정을 만들 수 있도록 기능 확장.
- **일정 확인 및 수정**: 작성한 일정을 최종 확인할 수 있으며, 순서를 즉시 변경 가능.

### 📍 리뷰 (리뷰 작성 및 관리)

- **리뷰 작성 알림**: 여행 일정이 끝난 후, **리뷰 작성**을 권장하는 알림을 앱 내에서 팝업으로 표시.
- 일정이 끝난 후 **3일 이내**에 리뷰를 작성하지 않으면, 팝업을 재표시하여 리뷰 작성을 유도.
- **여러 날의 리뷰 작성 가능**: 일정과 마찬가지로 하루 이상의 리뷰 작성 기능 제공.
- **리뷰 수정 및 관리**: 리뷰 데이터를 불러와 수정 후 업데이트 가능.

### 📍 설정 (유저 프로필 및 여행 성향 수정)

- **유저 프로필 수정**: 사용자의 여행 취향 및 개인 데이터를 수정 가능.
- **로그아웃**: 앱에서 로그아웃 기능 제공.

## ⚙️ Tech Stackl

- **Backend**: AWS S3, Lambda, API Gateway, DynamoDB, Cognito, SNS, CloudWatch
- **Frontend**: React Native, JavaScript, Figma
- **External**: Kakao API(Map, SSO)

## 📝 License

This project is licensed under the MIT License
