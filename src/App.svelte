<script lang="ts">
	export let name: string;
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
                        <div class="moduleDivider">
                            <span>web</span>
                        </div>
                        <!-- mock results -->
                        <div class="searchResult">
                            <div class="searchImg">
                                <img src="images/web.png">
                            </div>
                            <p>Go to google.com</p>
                        </div>
                        <div class="searchResult">
                            <div class="searchImg">
                                <img src="images/web.png">
                            </div>
                            <p>Generate shortlink for google.com</p>
                        </div>

                        <div class="moduleDivider">
                            <span>networking</span>
                        </div>
                        <div class="searchResult">
                            <div class="searchImg">
                                <img src="images/net.png">
                            </div>
                            <p>Resolve google.com to IP address</p>
                        </div>
                        <div class="searchResult">
                            <div class="searchImg">
                                <img src="images/net.png">
                            </div>
                            <p>List DNS records for google.com</p>
                        </div>
                        <!-- /mock results -->
                    </div>
                </div>
                <div class="right-col">
                    <div class="resultContainer">
                        <div class="blocky">
                            <p class="code">
                                Listing DNS records for www.google.com...
                            </p>
                            <h1 class="code">Records
                            </h1>
                                <table>
                                    <tr>
                                        <th>Type</th>
                                        <th>Domain Name</th>
                                        <th>TTL</th>
                                        <th>Address</th>
                                    </tr>
                                    <tr>
                                        <td>A</td>
                                        <td>google.com</td>
                                        <td>296</td>
                                        <td>142.251.32.100</td>
                                    </tr>
                                <!-- </table> -->
                                <!-- <h1 class="code">AAAA
                                </h1> -->
                                    <!-- <table>
                                        <tr>
                                            <th>Type</th>
                                            <th>Domain Name</th>
                                            <th>TTL</th>
                                            <th>Address</th>
                                        </tr> -->
                                        <tr>
                                            <td>AAAA</td>
                                            <td>google.com</td>
                                            <td>300</td>
                                            <td>2607:f8b0:4006:80e::2004
                                            </td>
                                        </tr>
                                    </table>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </body>
</main>

