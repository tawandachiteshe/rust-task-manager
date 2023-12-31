import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";

import { emit, listen } from '@tauri-apps/api/event'

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [sysInfo, setSysInfo] = useState<any>()
  const [sysInfoV2, setSystInfoV2] = useState<any>();

  useEffect(() => {
    listen("system-stats", (e) => {

      setSystInfoV2(e.payload)
    })
  })

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

    invoke("calculate_mem").then(mem => {

      setSysInfo(JSON.parse(mem as string))
    })
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    invoke("init_process").then(sysInfo => {

    })
  }, [])



  useEffect(() => {


    setInterval(() => {
      invoke("calculate_mem").then(mem => {

        setSysInfo(JSON.parse(mem as string))
      })
    }, 2000)


  }, [])

  return (
    <div className="container mx-auto px-4">

      <div className="mt-4">

        <div>
          <h3 className="text-2xl text-center text-gray-700">
            Average CPU Load
          </h3>
          <div className="text-5xl font-extrabold text-center">
            <span className={`bg-clip-text text-transparent bg-gradient-to-r from-pink-500 to-violet-500 from-${(Number(sysInfoV2?.cpus?.reduce((accumulator: any, currentValue: any) => accumulator + currentValue?.cpu_usage, 0)) / sysInfoV2?.cpus?.length).toFixed(1)}% to-100%`}>
              {(Number(sysInfoV2?.cpus?.reduce((accumulator: any, currentValue: any) => accumulator + currentValue?.cpu_usage, 0)) / sysInfoV2?.cpus?.length).toFixed(1)}
            </span>
          </div>

        </div>


        <div className="grid justify-center mt-4 w-full">
          <h3 className="text-2xl text-center text-gray-700">
            CPUs Load
          </h3>
          <div className="grid gap-2 w-full grid-cols-3 justify-center">
            {sysInfoV2?.cpus?.map((load?: any, index?: number) => {
              return (

                <div key={load?.name} className="w-24 h-24 grid justify-items-center rotate-180 bg-gray-200 dark:bg-gray-700">
                  <div className="grid justify-items-center bg-blue-500 text-center w-24 h-24" style={{ height: `${Number(load?.cpu_usage).toFixed(0)}%` }}>
                    <p className="text-yellow-50 p-3 self-center rotate-180 text-3xl font-boid">{(Number(load?.cpu_usage)).toFixed(0)}%</p>
                  </div>
                </div>
              )
            })}
          </div>
        </div>


        <div className="grid justify-center mt-4 w-full">
          <h3 className="text-2xl text-center text-gray-700">Memory usage</h3>
          <div className="w-72 mt-2 h-24 bg-gray-200 dark:bg-gray-700">
            <div className="grid justify-items-center bg-blue-500 w-72 h-24 align-middle rounded-bl" style={{ width: `${Number((Number(sysInfo?.mem?.free_size) / Number(sysInfo?.mem?.total_size)) * 100).toFixed(0)}%` }}>
              <div className="text-yellow-50 self-center flex-auto text-1xl font-boid">{sysInfo?.mem?.usage} / {sysInfo?.mem?.total}</div>
            </div>
          </div>
        </div>

      </div>






    </div>
  );
}

export default App;
