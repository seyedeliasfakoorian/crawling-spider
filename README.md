## Introduction

> A project built with rust that is a visual live browser simulation of the web crawling process.

_Can you help keep this open source service alive? **💖 Please like this project**_


### Usage

Using GUI, the usage of the **crawling-spider** can be completed by following the following steps.

> **_NOTE:_** We assume that you have **cargo** installed.

   1. Clone The Repository
   ```
     git clone https://github.com/seyedeliasfakoorian/crawling-spider.git && cd crawling-spider
   ```

   2. Install Packages
   ```
     cargo build
   ```

   3. Add to path
   ```
     cargo install --path .
   ```

   **How it Works:**

   After running the commands, you now have crawling-spider ready to use. Here is an example:


   ```
     crawling-spider randomsite.com
   ```

   Now, when the command is runned, you can go to [localhost:3000](http://localhost:3000/) and view the website that finded all the valuable public files.

   To keep up-to-date with the updates, you can run `git pull`.
