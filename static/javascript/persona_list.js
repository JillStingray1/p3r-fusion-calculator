window.onload = function () {
  DataTable.defaults.column.orderSequence = ["asc", "desc"];
  let table = new DataTable("#persona_table", {
    paging: false,
    bInfo: false,
  });
};
