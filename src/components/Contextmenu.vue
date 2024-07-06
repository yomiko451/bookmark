<template>
    <ol class="contextmenu" v-show="isShow" id="contextmenu">
        <li @click="cutText" @mousedown="getNoFocus">剪切</li>
        <li @click="copyText" @mousedown="getNoFocus">复制</li>
        <li @click="pasteText" @mousedown="getNoFocus">粘贴</li>
        <li @click="selectAll" @mousedown="getNoFocus">全选</li>
        <li @click="deleteText" @mousedown="getNoFocus">删除</li>
    </ol>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {readText, writeText} from '@tauri-apps/api/clipboard'


const isShow = ref<boolean>(false)
document.addEventListener('click', (e)=>{
    if (e.button === 0) {
        isShow.value=false
    }
})
document.oncontextmenu = (e) => {
    e.preventDefault()
    let element = e.target as HTMLElement
    if (element.tagName === 'INPUT') {
        const menu = document.getElementById('contextmenu')
        const height = document.documentElement.clientHeight
        const width = document.documentElement.clientWidth
        const rem = parseInt(window.getComputedStyle(document.documentElement).fontSize)
        if (menu) {
            menu.style.top = ''
            menu.style.left = ''
            menu.style.right = ''
            menu.style.bottom = ''
            if (e.clientX + (10*rem) >= width) {
                menu.style.right = (width - e.clientX) + 'px'
            } else {
                menu.style.left = e.clientX + 'px'
            }
            if (e.clientY + (12*rem) >= height) {
                menu.style.bottom = (height - e.clientY) + 'px'
            } else {
                menu.style.top = e.clientY + 'px'
            }
        }
        isShow.value = true
    }
}

async function copyText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart !== null && selectelement.selectionEnd !== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
    }
}

async function cutText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
    }
}

async function pasteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = await readText()
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + text + selectelement.value.substring(selectelement.selectionEnd)
    }
}

function selectAll() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        selectelement.selectionStart = 0
        selectelement.selectionEnd = selectelement.value.length
    }
}

function deleteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
    }
}

function getNoFocus(e: MouseEvent) {
    e.preventDefault()
}
</script>

<style scoped>
.contextmenu {
    position: absolute;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    width: 10rem;
    height: 12rem;
    background-color: var(--primary-color);
    list-style: none;
    padding: 0.5rem;
    border-width: 0.2rem;
    border-style: solid;
    border-top-color: var(--border-light-color);
    border-left-color: var(--border-light-color);
    border-bottom-color: var(--border-dark-color);
    border-right-color: var(--border-dark-color);
}
.contextmenu>li {
    font-size: 1.4rem;
    line-height: 2rem;
    padding: 0 0.5rem;
    cursor: pointer;
    transition: all 0.1s;
    user-select: none;
}
.contextmenu>li:hover {
    background-color: var(--secondary-color);
}
</style>