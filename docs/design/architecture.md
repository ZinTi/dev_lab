# architecture | 架构设计

## 项目结构

```bash
my-enterprise-app/ (Git Repository Root)
├── .gitignore
├── README.md               # 项目总览、构建说明、环境设置等
├── LICENSE
├── Makefile                # 可选：顶层构建/任务脚本
├── docker-compose.yml      # 可选：用于本地开发环境编排
│
├── api/                    # **核心共享接口定义 (契约优先)**
│   ├── proto/              # gRPC Protocol Buffer (.proto) 文件
│   │   ├── auth/           # 认证服务接口
│   │   ├── service1/       # 业务服务1接口
│   │   ├── service2/       # 业务服务2接口
│   │   └── common/         # 跨服务共享消息类型
│   └── openapi/            # RESTful API OpenAPI/Swagger 规范
│       ├── gateway/        # 网关暴露的 REST API 定义
│       └── ...             # (可选) 其他直接暴露的 REST 服务定义
│
├── common/                 # **跨模块共享代码/工具 (谨慎使用)**
│   ├── config/             # 配置加载库 (支持 YAML/JSON)，各语言实现
│   │   ├── go/
│   │   ├── java/
│   │   └── cpp/
│   ├── utils/              # 通用工具类/函数库 (按语言分)
│   │   ├── go/
│   │   ├── java/
│   │   └── cpp/
│   └── ...                 # 其他真正通用的共享代码
│
├── services/               # **所有可独立部署的运行时服务 (核心变更点)**
│   │
│   ├── gateway/            # **API 网关服务 (Go)**
│   │   ├── cmd/            # 入口 (main.go)
│   │   ├── internal/       # 私有实现 (handler, middleware, grpcclient, config)
│   │   ├── pkg/            # 内部可复用包
│   │   ├── configs/        # 网关配置文件
│   │   ├── Dockerfile
│   │   └── go.mod
│   │
│   ├── auth/               # **认证服务 (例如 Go/Java)**
│   │   ├── cmd/ or src/    # 根据语言选择结构 (Go: cmd/main.go; Java: src/main/java...)
│   │   ├── internal/ or ... # 私有实现
│   │   ├── configs/        # 认证服务配置文件
│   │   ├── migrations/     # 数据库迁移脚本
│   │   ├── Dockerfile
│   │   └── go.mod / pom.xml / ... # 构建文件
│   │
│   ├── service1/           # **业务服务1 (例如 Java)**
│   │   ├── src/            # Java 源代码 (Maven/Gradle 结构)
│   │   ├── configs/
│   │   ├── Dockerfile
│   │   └── pom.xml
│   │
│   ├── service2/           # **业务服务2 (例如 Go)**
│   │   ├── cmd/
│   │   ├── internal/
│   │   ├── configs/
│   │   ├── Dockerfile
│   │   └── go.mod
│   │
│   └── ...                 # **其他服务 (如定时任务服务、消息处理服务等)**
│
├── clients/                # **客户端应用**
│   ├── desktop/            # 桌面客户端
│   └── web/                # 网页端
│
├── configs/                # **全局或基础架构配置**
│   ├── env/                # 环境变量文件
│   ├── k8s/                # Kubernetes 部署文件 (每个服务一个子目录或文件)
│   └── ...                 # 其他全局配置
│
├── scripts/                # **实用脚本**
│   ├── generate/           # 代码生成脚本 (protoc, openapi-generator)
│   ├── deploy/             # 部署脚本
│   ├── test/               # 集成/端到端测试脚本
│   └── tools/              # 开发工具脚本
│
├── docs/                   # **项目文档**
│   ├── design/
│   ├── api/
│   ├── deployment/
│   └── ...
│
└── test/                   # **非单元测试 (单元测试在各服务/客户端内部)**
    ├── integration/        # 集成测试
    └── e2e/                # 端到端测试
```
