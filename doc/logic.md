# 流程图

```mermaid
graph TD
    A[页面加载] --> B{初始化}
    B --> B1[初始化PJAX]
    B --> B2[初始化翻译系统]
    B --> B3[初始化深色模式]
    B --> B4[初始化目录生成]
    B --> B5[自定义PJAX事件监听]
    A --> C[加载Markdown内容]
    C --> D{解析Markdown}
    D --> E[生成HTML内容]
    E --> F[处理链接路径]
    F --> G[插入DOM]
    G --> H[启动加载动画]
    H --> I[等待动画完成]
    I --> J[更新页面状态]
    J --> K[错误处理]
    K -->|成功| J
    K -->|失败| L[显示错误信息]
    A --> M[用户交互]
    M --> N{点击链接}
    N --> O[触发PJAX请求]
    O --> C
```

```mermaid
classDiagram
    class MarkdownRenderer{
        +string mdPath
        +fetchMD()
        +parseContent()
        +generateOutline()
        +updateDOM()
    }
    class PJAXHandler{
        +handleNavigation()
        +animateTransition()
    }
    class DarkMode{
        +toggleMode()
    }
    class Translator{
        +translateContent()
        +detectLanguage()
    }
    MarkdownRenderer --> PJAXHandler: 使用
    MarkdownRenderer --> DarkMode: 集成
    MarkdownRenderer --> Translator: 集成
```

## 核心流程说明

1. **初始化阶段**：加载所有依赖库并配置核心功能模块
2. **内容加载**：通过fetch获取Markdown文件内容并解析
3. **渲染流程**：生成HTML内容 -> 处理路径 -> 更新DOM -> 动画过渡
4. **交互处理**：通过PJAX实现无刷新导航，触发新的加载流程
5. **异常处理**：在关键节点捕获错误并显示友好提示
6. **增强功能**：翻译系统与暗黑模式通过事件总线集成
