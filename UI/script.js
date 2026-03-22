async function findInPi() {
    const type = document.getElementById('searchType').value;
    const query = document.getElementById('searchQuery').value;
    const res = document.getElementById('result');
    const status = document.getElementById('status');

    if (!query) return;
    status.innerText = "Searching local data...";
    res.innerText = "";

    try {
        const response = await fetch(`http://127.0.0.1{type}/${encodeURIComponent(query)}`);
        const data = await response.json();
        
        if (data.calculating) {
            status.innerText = "Not in local file. Calculating more digits...";
            // Optional: Poll again or wait for long-poll response
        }
        
        status.innerText = data.found ? "Success!" : "Not found.";
        res.innerText = data.message;
    } catch (err) {
        res.innerText = "Backend Connection Error.";
    }
}
