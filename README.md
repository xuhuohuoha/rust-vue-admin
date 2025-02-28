# Rust-Vue-Admin

## 1 项目简介

适用于全栈快速构建项目的业务框架，基于 `Rust` + `Vu3` 构建，自动权限配置体系、通用业务功能，支持扩展及自定义。

## 2 安装指南

### 2.1 克隆仓库
 bash git clone https://github.com/xuhuohuoha/rust-vue-admin.git

### 2.2 安装依赖

#### 2.2.1 前端依赖

基于 Vue3 + Vite + Pinia + Router + Naive UI 构建，详见项目中`package.json`

npm install
#### 2.2.2 后端依赖

基于 Rust + cmake 构建，详见项目中`Cargo.toml`

1. install cmake

2. cargo run

## 3 使用说明

### 3.1 项目配置

第一步：恢复数据库（mysql）,脚本位置 bxy\blunka-bpms\db\test.sql

第二步：修改配置文件`.env`，文件位置 bxy\blunka-bpms\.env

第三步：修改配置文件`config.toml`，文件位置 bxy\blunka-bpms\config\config.toml

第四步：启动后端项目 cargo run

第五步：启动前端项目 yarn dev

第六步：访问项目：http://localhost:3000

## 4 贡献指南

## 5 许可证

MIT License

## 6 联系

可以通过 icefrozen2013@hotmail.com 联系我





