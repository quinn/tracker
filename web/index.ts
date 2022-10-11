// import {create} from "./component"

// console.log('hello from ts')

// create('my-paragraph')

import Alpine from 'alpinejs'
// import {useSyncExternalStore} from "use-sync-external-store";

import {getAllUsers} from "./api";

// console.log(useSyncExternalStore)
// @ts-ignore
// window.Alpine = Alpine

Alpine.start()

async function main()
{
    await getAllUsers()
}

main()
