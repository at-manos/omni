<script>
    import {invoke} from '@tauri-apps/api/tauri'
    import { onMount } from "svelte";
    // const invoke = window.__TAURI_IPC__.invoke
    async function run(search){
        return JSON.parse(await t(search));
        // console.log(test);
    }
    // async
    async function t(search){
        let test = await (invoke("search", {search: search}))
        return test;
    }

    var test = run().then(val=>{return val});
    console.log(test);
    var setInnerHTML = function(elm, html) {
        elm.innerHTML = html;
        Array.from(elm.querySelectorAll("script")).forEach( oldScript => {
            const newScript = document.createElement("script");
            Array.from(oldScript.attributes)
            .forEach( attr => newScript.setAttribute(attr.name, attr.value) );
            newScript.appendChild(document.createTextNode(oldScript.innerHTML));
            oldScript.parentNode.replaceChild(newScript, oldScript);
        });
    }
    onMount(async ()=>{
        var input = document.querySelector(".searchinput");
        input.addEventListener("keyup", async function(event) {
            // if(event.keyCode === 13){
                console.log(input.value);
                test = await run(input.value).then(val=>{return val});
                console.log(test);
                setInnerHTML(document.querySelector(".resultContainer"), "");
            // }
        });
        // var but = document.querySelector(".searchResult");
        // but.addEventListener("click", function(event) {
        //         console.log("test");
        //         // test = run(input.value).then(val=>{return val});
        // });
    })
    async function getResult(search, module, uid){
        let test = await (invoke("result", {search: search, module: module, uid: uid}));
        setInnerHTML(document.querySelector(".resultContainer"), await test);
        return test;
    }
    function handleClick(e) {
        document.querySelector(".resultContainer").innerHTML = "";
		let path = e.composedPath();
        let res = getResult(document.querySelector(".searchinput").value, path[1].id, path[0].id).then(val=>{return val});
        console.log(res);
	}
</script>

<main>
    <body>
        <div class="container">
            <div class="topbar">
                <div class="searchIcon">
                    <img src="/images/search.png">
                </div>
                <div class="search">
                    <input type="text" class="searchinput" placeholder="Enter your search...">
                </div>
            </div>
            <div class="content">
                <div class="left-col">
                    <div class="results">
                        <!-- divider -->
                        {#await test then results}

                            {#each results as mod}
                            <div class="moduleDivider" >
                                <span>{mod.name}</span>
                            </div>
                                {#each Object.entries(mod.results) as [k,v]}
                                <div class="searchResult" id={mod.name} on:click={handleClick}>
                                    <div class="searchImg">
                                        <img src="images/web.png">
                                    </div>
                                    <p id={k} class="searchP">{v}</p>
                                </div>
                                {/each}
                            {/each}
                        {/await}
                        <!-- mock results -->
                        
                        <!-- /mock results -->
                    </div>
                </div>
                <div class="right-col">
                    <div class="resultContainer">
                        <!-- resultright -->
                    </div>
                </div>
            </div>
        </div>
    </body>
</main>

