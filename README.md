This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.

## AI提示词生成简单桌面端应用 Next.js + Tauri + Tare

### 对话流程
```md
1. 使用Next.js和tauri创建一个新项目
2. npm run tauri build配置 - 查看nextjs文档
3. 生成一个图片压缩功能
4. 增加图片上传压缩的UI
5. 文件不保存在tmp目录，直接保存在原本文件的路径
6. 图片压缩后保存在上传源文件所在的目录位置，不要保存在项目中
7. 使用文件名不带图片格式+随机hash值作为文件名
8. 如何使用tauri将项目打成应用
```

### 命令
```md
1. npx create-next-app@latest --typescript  // next创建
2. pnpm run dev // nextjs启动
3. pnpm tauri init // tauri安装
4. pnpm run tauri dev    // 客户端启动
5. pnpm run tauri build  // 打包
```