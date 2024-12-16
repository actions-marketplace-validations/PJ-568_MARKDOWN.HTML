# 程序逻辑树状图

1. **IIFE 包裹 (避免全局变量污染)**
    - 使用立即执行函数表达式 (IIFE) 包裹代码，避免全局变量污染。

2. **初始化**
    - **`initialize()`**：初始化所有功能
      - **`initDarkmode()`**：初始化深色模式插件
      - **`initHomeBtn()`**：初始化返回首页按钮
      - **`initAboutBtn()`**：初始化关于按钮
      - **文档解析功能**：

        **`initContent()`**：初始化页面内容
        - **`showMarkdown()`**：更新页面的 Markdown 内容
          - **`getPValue()`**：获取 URL 参数 `p` 的值
          - **`decodeMDPath()`**：将 URL 参数 `p` 转换为绝对路径
          - **`formatMDPath()`**：格式化路径
          - **`getMarkdown()`**：读取并解析指定的 Markdown 文件
            - **`marked.parse()`**：解析 Markdown
            - **处理内部链接**
          - **更新页面内容**
          - **`customPushState()`**：自定义推送浏览器历史状态
      - **`initTranslate()`**：初始化翻译系统
      - **`initPJAX()`**：初始化 PJAX，实现页面无刷新加载
      - **初始化自定义 PJAX**：
        - **`initCustomPJAXResponse()`**: 覆写 PJAX 处理响应的函数，实现 Markdown 渲染。
        - **`initCustomPJAXEventListener()`**: 初始化自定义 PJAX 事件监听器，处理点击事件。

3. **事件监听**
    - **`DOMContentLoaded` 事件**：网页加载完毕后触发初始化
      - 调用 `initialize()` 函数
    - **`pjax:send` 事件**：PJAX 开始时执行的函数
      - **`startLoad()`**：开始加载动画
    - **`pjax:complete` 事件**：监听 PJAX 完成后
      - **`initCustomPJAX()`**：重新加载内容
    - **`click` 事件委托**
      - **返回上一页 (`#back-btn`)**：调用 `window.history.back()`
      - **刷新页面 (`#refresh-btn`)**：调用 `window.location.reload()`
      - **切换深色模式 (`#dark_b`)**：调用 `darkmode.toggle()`
      - **切换语言 (`#translate-switch`)**：调用 `translate.changeLanguage()`
    - **`showMarkdownEnded()` 函数**：文档内容加载完毕时触发
      - **`refreshTranslate()`**：刷新翻译
      - **`infoTranslate()`**：检查并提示翻译
      - **`translate.execute()`**：执行翻译
      - **`endLoad()`**：结束加载动画
      - **添加 `loaded` 类**
      - **平滑滚动到顶部**：调用 `window.scrollTo()`
    - **`onpopstate` 事件**：浏览器返回时触发
      - **更新层级 (`level`)**

4. **加载动画相关**
    - **`startLoad()`**：开始加载动画，显示加载进度条
    - **`endLoad()`**：结束加载动画，隐藏加载进度条

5. **文档解析功能**
    - **`getPValue()`**：获取 URL 参数 `p` 的值
    - **`getDirectory()`**：获取文件所在目录
    - **`getPath()`**：获取当前页面的 Markdown 文件路径
    - **`decodeMDPath()`**：将 URL 参数 `p` 转换为绝对路径
    - **`formatMDPath()`**：格式化路径
    - **`isRelativePath()`**：判断路径是否为相对路径
    - **`handleError()`**：处理加载错误，显示错误信息
    - **`showMarkdown()`**：更新页面的 Markdown 内容
      - **`getMarkdown()`**：读取并解析指定的 Markdown 文件
        - **`marked.parse()`**：解析 Markdown
        - **处理内部链接**
        - **`generateOutline()`**：生成目录结构

6. **PJAX 相关**
    - **`initPJAX()`**：初始化 PJAX
    - **`initCustomPJAXResponse()`**：覆写 PJAX 处理响应的函数
    - **`initCustomPJAXEventListener()`**：初始化自定义 PJAX 事件监听器

7. **深色模式相关**
    - **`initDarkmode()`**：初始化深色模式插件

8. **翻译系统**
    - **`initTranslate()`**：初始化翻译系统
    - **`infoTranslate()`**：检查并提示翻译
    - **`refreshTranslate()`**：刷新翻译
    - **`translate.execute()`**：执行翻译

9. **自定义推送状态**
    - **`customPushState()`**：自定义推送浏览器历史状态

10. **初始化内容**
    - **`initContent()`**：初始化页面内容
      - **`showMarkdown()`**：更新页面的 Markdown 内容

11. **初始化按钮**
    - **`initHomeBtn()`**：初始化返回首页按钮
    - **`initAboutBtn()`**：初始化关于按钮
