const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetPort;
let greetMsgEl;
//打开窗口暂时找不到根据参数传递指定窗口，只能一个函数打开一个窗口
async function openNewWindow(){
  try {
    await invoke('open_new_window');
  } catch (error) {
    console.error('Error opening new window:', error);
  }
};
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value ,port: greetPort.value});
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetPort = document.querySelector("#greet-port");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
	openNewWindow();
	//这是一个打开url的接口，不是打开本地html的
//	window.__TAURI__.shell.open('http://127.0.0.1:1430/rulecfg.html');
  });
});
