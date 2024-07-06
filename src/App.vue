<template>
    <div class="container win2000-1 win2000-2">
        <div class="titlebar">
            <div class="icon">
                <img :src="icon" alt="icon">
            </div>
            <span class="name" data-tauri-drag-region>bookmark</span>
            <div class="titlebar-button win2000-1 win2000-2" @click="appWindow.minimize">
                <img :src="minimize" alt="minimize" />
            </div>
            <div class="titlebar-button win2000-1 win2000-2" @click="appWindow.toggleMaximize">
                <img :src="maximize" alt="maximize" />
            </div>
            <div class="titlebar-button win2000-1 win2000-2" @click="closeWindow">
                <img :src="close" alt="close" />
            </div>
        </div>
        <div class="botton-area">
            <div class="botton win2000-1 win2000-2" @click="addBookMark">新建书签</div>
            <p class="bookmark-count">{{ bookMarkCount }}</p>
            <div class="botton win2000-1 win2000-2" @click="deleteBookMark">删除书签</div>
        </div>
        <ol class="content-area">
            <li @dblclick="modifyBookMark(item)" @click="selectedIndex=index" :class="selectedIndex==index?'selected':''"
                v-for="item,index in bookMarkArr" :key="item.id">
                <p>{{ `[${index + 1}]《${item.name}》-> ${item.page}` }}</p>
                <p>{{ '备注：' + item.description }}</p>
            </li>
        </ol>
        <div class="input-subwindow win2000-1 win2000-2" v-show="showInputSubwindow">
            <p class="input-title">请输入书签信息</p>
            <div class="input-content">
                <div>
                    <span class="input-text">名称</span>
                    <input class="win2000-1" type="text" placeholder="请输入名称" v-model="nameText">
                </div>
                <div>
                    <span class="input-text">页码</span>
                    <input class="win2000-1" type="text" placeholder="请输入页码" v-model="pageText">
                </div>
                <div>
                    <span class="input-text">备注</span>
                    <input class="win2000-1" type="text" placeholder="请输入备注" v-model="descriptionText" @keydown.enter="showAddButton?confirmAddBookMark():confirmModifyBookMark()">
                </div>
                <div class="footbar">
                    <div class="win2000-1 win2000-2" v-if="showAddButton" @click="confirmAddBookMark()">新建</div>
                    <div class="win2000-1 win2000-2" v-else @click="confirmModifyBookMark">修改</div>
                    <div class="win2000-1 win2000-2" @click="showInputSubwindow=false">取消</div>
                </div>
            </div>
        </div>
        <div class="dialog-subwindow win2000-1 win2000-2" v-show="showDialogSubwindow">
            <p class="dialog-title">提醒！</p>
            <div class="dialog-content">
                <p class="dialog-text">{{ showDeleteButton?'确定要删除书签9吗？':'请输入有效的书签信息！' }}</p>
                <div class="dialog-footbar">
                    <div v-if="showDeleteButton" class="win2000-1 win2000-2" @click="confirmDeleteBookMark">删除</div>
                    <div class="win2000-1 win2000-2" @click="showDialogSubwindow = false">{{ showDeleteButton?'取消':'确定' }}</div>
                </div>
            </div>
        </div>
        <Contextmenu />
        <audio ref="audio" :src="sound"></audio>
    </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import { appWindow } from '@tauri-apps/api/window'
import close from './assets/close.svg'
import minimize from './assets/minimize.svg'
import maximize from './assets/maximize.svg'
import icon from './assets/icon.svg'
import sound from './assets/sound.wav'
import Contextmenu from './components/Contextmenu.vue';
import { invoke } from '@tauri-apps/api';

//初始化
onMounted(async ()=>{
    bookMarkArr.value = await invoke('init')
    appWindow.show()
})


// 定义书签接口
interface BookMark {
    name: string,
    page: string
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

const audio = ref<HTMLAudioElement | null>()
const showInputSubwindow = ref(false)
const showDialogSubwindow = ref(false)

const nameText = ref('')
const pageText = ref('')
const descriptionText = ref('')
const showAddButton = ref(true)
const showDeleteButton = ref(true)

const addBookMark = () => {
    nameText.value = ''
    pageText.value = ''
    descriptionText.value = ''
    showAddButton.value = true
    showInputSubwindow.value = true
}
const confirmAddBookMark = async () => {
    const name = nameText.value
    const page = pageText.value
    const description = descriptionText.value
    if (name === '' || page === '' ) {
        showDeleteButton.value = false
        audio.value?.play()
        showDialogSubwindow.value = true
    } else {
        const nweBookMark: BookMark = await invoke('add_bookmark', {name,page,description})
        bookMarkArr.value.push(nweBookMark)
        showInputSubwindow.value = false
    }    
}

const confirmModifyBookMark = async () => {
    const name = nameText.value
    const page = pageText.value
    const description = descriptionText.value
    if (name === '' || page === '' ) {
        showDeleteButton.value = false
        audio.value?.play()
        showDialogSubwindow.value = true
    } else {
        const id = bookMarkArr.value[selectedIndex.value].id
        const nweBookMark: BookMark = await invoke('modify_bookmark', {name,page,description,id})
        bookMarkArr.value[selectedIndex.value] = nweBookMark
        showInputSubwindow.value = false
    }
}

//双击修改书签信息
const modifyBookMark = (item: BookMark) => {
    nameText.value = item.name
    pageText.value = item.page.toString()
    descriptionText.value = item.description
    showAddButton.value = false
    showInputSubwindow.value = true
}

const deleteBookMark = () => {
    showDeleteButton.value = true
    showDialogSubwindow.value = true
    audio.value?.play()
}
const confirmDeleteBookMark = () => {
    const id = bookMarkArr.value[selectedIndex.value].id
    invoke('delete_bookmark', {id})
    bookMarkArr.value.splice(selectedIndex.value,1)
    selectedIndex.value = 0
    showDialogSubwindow.value = false
}
// 关闭窗口时候更新id
const closeWindow = async () => {
    await invoke('update_id')
    appWindow.close()
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
    padding-bottom: 1rem;
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
.win2000-1, li {
    border-width: 0.2rem;
    border-style: solid;
}
.win2000-2, li:hover {
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
.input-subwindow, .dialog-subwindow {
    top: 30%;
    left: 50%;
    transform: translateX(-50%);
    position: absolute;
    display: grid;
    grid-template-rows: 2.4rem 1fr;
}
.input-subwindow {
    width: 80%;
    height: 16rem;
}
.input-title, .dialog-title {
    background: linear-gradient(to right, rgb(17,39,99), rgb(166,202,236));
    color: white;
    line-height: 2.4rem;
    font-size: 1.4rem;
    padding: 0 0.6rem;
    align-items: center;
    margin: 0;
    user-select: none;
}
.input-content, .dialog-content {
    background-color: var(--primary-color);
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
}
.input-content>div, .dialog-content>div {
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
.footbar, .dialog-footbar {
    display: flex;
    justify-content: space-evenly;
}
.footbar>div, .dialog-footbar>div {
    height: 2.4rem;
    width: 6rem;
    font-size: 1.4rem;
    text-align: center;
    line-height: 2.4rem;
    user-select: none;
}
.footbar>div:active, .dialog-footbar>div:active {
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
    line-height: 3rem;
    font-size: 1.4rem;
    color: white;
    flex: 1;
    margin: 0 0.6rem;
}
.dialog-subwindow {
    height: 10rem;
    width: 80%;
}
.dialog-text {
    font-size: 1.4rem;
    text-align: center;
    margin: 0;
    user-select: none;
}
</style>