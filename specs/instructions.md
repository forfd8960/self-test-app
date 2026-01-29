# Instructions


## Requirements

**Self test application** that allows users to upload learning materials (PDF, DOCX, TXT, etc.) and generates personalized test questions (multiple-choice and fill-in-the-blank) using AI. The app should provide feedback based on user performance.:

- User registration and login to the app (using JSON Web Token authentication)
- Users upload learning materials (PDF/DOCX/TXT, etc.)
- Users set personalized question generation parameters: for example, 100 multiple-choice questions (single or multiple choice), and 100 fill-in-the-blank questions.
- After the settings are complete,
- The system starts generating questions and answers and stores them in the database.
- After generation, the user views the generated test page (list of generated questions).
- The user enters the test page and begins answering the questions.
- After the user completes the test, the system first provides a score and constructive feedback based on the answers and learning materials, such as which content needs reviewing, which content is not well understood, and which areas were answered well.
- Users can view their historical test records, including the score, answer details, and system feedback for each test.

- 用户注册登录 app(json web token 认证)
- 用户上传学习资料(pdf/docx/txt 等等)
- 用户设置个性化的问题生成: 比如 100 道选择题(单选 or 多选), and 100道 填空题。
- 设置完成后
- 系统开始生成问题与答案, 并存储到 DB 里面。
- 生成之后用户查看生成的测试页面(生成的题目列表)
- 用户进入测试页面, 开始答题。
- 当用户完成答题后，系统首先给出评分，以及根据答案以及学习资料给出建设性的反馈, 比如哪些内容需要在复习，理解不到位的内容有哪些，答的好的方面。
- 用户可以查看历史测试记录，包含每次测试的成绩，答题详情，系统反馈等。

## Technology Stack

- Backend: Rust (Axum, SQLx for database interaction, async-openai for AI client)
- AI Service: MiniMax for question generation: Model: MiniMax-M2.1, BaseURL: https://api.minimaxi.com/v1.
- Frontend: TypeScript (React framework) + Zustand +  Vite + npm + Tailwind CSS
- Use the latest dependencies where possible.
- Database: PostgreSQL.
- Authentication: JSON Web Tokens (JWT)
- File Storage: Local file system.
- Question Generation: Integrate with an AI service (e.g., MiniMax) for question generation based on uploaded materials.
- Testing: Unit tests and integration tests for both backend and frontend components.
- Deployment: local running.

## Limitations

- User can not upload files which larger than 5MB.
- The AI model may have rate limits; implement retries with exponential backoff for robustness.
- Set the user level rate limit for AI requests to 10 requests per minute.
- The AI calls for question generation may be long-running; implement background job processing with polling for status updates.
- JWT tokens should exipre after 6 huors.

## Database

- Use PostgreSQL for data persistence.
- the connect string is: postgres://postgres:postgres@localhost:5432/selftestapp

## UI Design

- Follow Google's Material Design 3 (M3) standards.
- Build the UI for Self Test Application using React with the latest Material library.
- Components: Include a Navigation Rail/Drawer, Top App Bar, and Elevating Action Buttons.
- Visuals: Follow the M3 color system (Primary, Secondary, Tertiary) with support for light/dark mode.
- Usability: Prioritize accessibility (ARIA labels), responsive layouts (mobile/desktop), and intuitive spacing (Material spacing scale).