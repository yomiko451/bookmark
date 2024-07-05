<template>
    <div class="container">
        <div data-tauri-drag-region class="titlebar">
            <div class="icon">
                <img :src="icon" alt="icon" data-tauri-drag-region>
            </div>
            <span class="name" data-tauri-drag-region>bookmark</span>
            <div class="titlebar-button" @click="appWindow.minimize">
                <img :src="minimize" alt="minimize" />
            </div>
            <div class="titlebar-button" @click="appWindow.toggleMaximize">
                <img :src="maximize" alt="maximize" />
            </div>
            <div class="titlebar-button" @click="appWindow.close">
                <img :src="close" alt="close" />
            </div>
        </div>


        <div class="botton-area">
            <div class="botton" @click="addBookMark">新建书签</div>
            <p class="bookmark-count">{{ bookMarkCount }}</p>
            <div class="botton">删除书签</div>
        </div>
        <ol class="content-area">
            <li @click="selectedIndex=index" :class="selectedIndex==index?'selected':''"
                v-for="item,index in bookMarkArr" :key="item.id">
                <p>{{ `[${index + 1}]《${item.name}》-> 第${item.page}页` }}</p>
                <p>{{ '备注：' + item.description }}</p>
            </li>
        </ol>
        <div class="input-subwindow" v-show="showInputSubwindow">
            <p class="input-title">请输入书签信息</p>
            <div class="input-content">
                <div>
                    <span class="input-text">标题</span>
                    <input type="text" placeholder="请输入标题">
                </div>
                <div>
                    <span class="input-text">页码</span>
                    <input type="text" placeholder="请输入页码">
                </div>
                <div>
                    <span class="input-text">备注</span>
                    <input type="text" placeholder="请输入备注">
                </div>
                <div class="footbar">
                    <div>确定</div>
                    <div @click="showInputSubwindow=false">取消</div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { appWindow } from '@tauri-apps/api/window'
import close from './assets/close.svg'
import minimize from './assets/minimize.svg'
import maximize from './assets/maximize.svg'
import icon from './assets/icon.svg'
// 定义书签接口
interface BookMark {
    name: string,
    page: number
    description: string,
    created_at: string,
    updated_at: string,
    id: string,
}
// 定义书签数组
const bookMarkArr = ref<BookMark[]>([])
// 定义选中索引
const selectedIndex = ref(0)

//定义书签数量
const bookMarkCount = computed(()=>{
    const len = bookMarkArr.value.length
    return len>=10?`${len}/99`:`0${len}/99`
})

const showInputSubwindow = ref(false)

const addBookMark = () => {
    showInputSubwindow.value = true
}


// const inputSubwindow = ref<HTMLDivElement | null>(null)
// onMounted(()=>{
//     inputSubwindow.value!.ondragover = (e) => e.preventDefault()
// })







//TODO:记得删除测试代码
const test: BookMark = {
    name: '我的精神家园',
    page: 55,
    description: 'p50:第三章',
    created_at: '2021-01-01',
    updated_at: '2021-01-01',
    id: '1',
}
for (let i = 1;i<=50;i++) {
    bookMarkArr.value.push(test)
}
</script>

<style scoped>
.container {
    display: grid;
    height: 100vh;
    width: 100vw;
    background-color: var(--primary-color);
    grid-template-rows: 3rem 6rem 1fr;
    position: relative;
    box-sizing: border-box;
}
.botton-area {
    display: flex;
    align-items: center;
    justify-content: space-evenly;
}
.botton {
    height: 3rem;
    width: 10rem;
    font-size: 1.6rem;
    text-align: center;
    line-height: 3rem;
    user-select: none;
    background-color: var(--primary-color);
}
.botton,li,.input-subwindow,input,.footbar>div,.container,.titlebar-button {
    border-width: 0.2rem;
    border-style: solid;
}
.botton,li:hover,.input-subwindow,.footbar>div,.container,.titlebar-button {
    border-top-color: var(--border-light-color);
    border-left-color: var(--border-light-color);
    border-bottom-color: var(--border-dark-color);
    border-right-color: var(--border-dark-color);
}
.botton:active {
    border-color: var(--border-dark-color);
    outline: 0.2rem dotted var(--border-dark-color);
    outline-offset: -0.8rem;
}
.bookmark-count {
    font-size: 1.6rem;
    user-select: none;
}
.content-area {
    margin: 0;
    overflow: scroll;
    padding: 0 1rem 1rem 1rem;
}
li {
    list-style: none;
    border-color: var(--primary-color);
    margin: 0.4rem 0;
}
.selected,input {
    background-color: var(--secondary-color);
    border-top-color: var(--border-dark-color);
    border-left-color: var(--border-dark-color);
    border-bottom-color: var(--border-light-color);
    border-right-color: var(--border-light-color);
}
.content-area p {
    font-size: 1.6rem;
    margin: 0.4rem 1rem;
    user-select: none;
}
.content-area>li>p:last-child {
    font-style: italic;
}
.input-subwindow {
    top: 30%;
    left: 50%;
    transform: translateX(-50%);
    position: absolute;
    width: 80%;
    height: 16rem;
    display: grid;
    grid-template-rows: 2.4rem 1fr;
}
.input-title {
    background: linear-gradient(to right, rgb(17,39,99), rgb(166,202,236));
    color: white;
    line-height: 2.4rem;
    font-size: 1.4rem;
    padding: 0 0.6rem;
    align-items: center;
    margin: 0;
    user-select: none;
}
.input-content {
    background-color: var(--primary-color);
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
}
.input-content>div {
    display: flex;
    align-items: center;
    margin: 0 1rem;
}
.input-text {
    font-size: 1.4rem;
    margin-right: 1rem;
    user-select: none;
}
input {
    background-color: var(--secondary-color);
    outline: none;
    flex: 1;
    font-size: 1.4rem;
}
.footbar {
    display: flex;
    justify-content: space-evenly;
}
.footbar>div {
    height: 2.4rem;
    width: 6rem;
    font-size: 1.4rem;
    text-align: center;
    line-height: 2.4rem;
    user-select: none;
}
.footbar>div:active {
    border-color: var(--border-dark-color);
    outline: 0.2rem dotted var(--border-dark-color);
    outline-offset: -0.6rem;
}
.titlebar {
  height: 3rem;
  background: linear-gradient(to right, rgb(17,39,99), rgb(166,202,236));
  user-select: none;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding: 0 0.6rem;
}
.titlebar-button {
  background-color: var(--primary-color);  
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 1.8rem;
  height: 1.8rem;
  margin-left: 0.2rem;
}
.titlebar>.icon {
    height: 2rem;
    width: 2rem;
}
.titlebar>.icon>img {
    width: 100%;
}
.titlebar>.name {
    font-size: 1.4rem;
    color: white;
    flex: 1;
    margin: 0 0.6rem;
}
</style>