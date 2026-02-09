

window.onload = function () {
    DataTable.defaults.column.orderSequence = ['asc', 'desc'];
    let reverse = new DataTable ( "#reverse-fusions", {
        paging: false,
        bInfo: false,
    });
    let forward = new DataTable ( "#forward-fusions", {
        paging: false,
        bInfo: false,
    });
}

function show_fusions(event, table_name) {
    let tabs = document.getElementsByClassName("tabcontent");
    for (let i = 0; i < tabs.length; i++) {
        tabs[i].style.display = "none";
    }
    let tab_buttons = document.getElementsByClassName("tabbutton active");
    for (let i = 0; i < tab_buttons.length; i++) {
        tab_buttons[i].classList.remove("active");
    }
    document.getElementById(table_name).style.display = "block";
    event.currentTarget.classList.add("active");
}