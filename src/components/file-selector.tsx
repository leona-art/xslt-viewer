import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import {UnlistenFn} from "@tauri-apps/api/event"
import { open } from "@tauri-apps/api/dialog";
import { Accessor, Setter, createEffect, createSignal, onCleanup, onMount } from "solid-js";

export default function (props: { title: string, ext?: string, setContent?: Setter<string> }) {
    const [path, setPath] = createSignal("")
    const [err, setErr] = createSignal<string | undefined>(undefined)
    let unlisten:UnlistenFn|undefined=undefined

    // ファイル選択ダイアログを開く
    async function openDialog() {
        const result = await open({ filters: [{ name: props.title, extensions: [props.ext ?? "*"] }] })
        if (result && !Array.isArray(result)) {
            setPath(result)
        }
    }
    async function reset() {
        await invoke("unwatch_file", { ext: props.ext })
        setPath("")
    }
    onMount(async () => {
        if (props.setContent) {
            unlisten = await appWindow.listen<string>(`change_${props.title}`, ({ payload }) => {
                props.setContent!(payload)
            })
        }
    })
    onCleanup(()=>{
        if(unlisten)unlisten()
    })
    createEffect(async()=>{
        // ファイルの拡張子をチェック
        if (path().endsWith(`.${props.ext}`)) {
            if (props.setContent) {
              invoke("watch_file", { path: path(), title:props.title }).catch(e=>console.log(e))
  
            }
            setErr(undefined)
          } else {
            setErr(`${props.ext} ファイルを選択してください`)
          }
    })
    return (
        <div>
        <label>{props.title}</label>
        <input value={path()} placeholder={err()} onChange={e=>setPath(e.currentTarget.value)}/>
        <button onClick={openDialog}>select</button>
        <button onClick={reset}>reset</button>
        <button onClick={() => invoke("open_file", { path: path() })} disabled={path()===""}>open</button>
      </div>
    )
}