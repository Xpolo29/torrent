//https://tauri.app/v1/guides/features/command/
// import { invoke } from '@tauri-apps/api/tauri'
const { invoke } = window.__TAURI__.tauri;

// #####################
// ##### FUNCTIONS #####
// function to add a message
function addMessage(message, divName, setTimeoutTime = 0, id = message) {
  //console.log('addMessage called to show: ', message);
  var messageElement = document.createElement('p');
  messageElement.textContent = message;
  messageElement.id = id;
  divName.appendChild(messageElement);

  if (setTimeoutTime === 0) return;
  setTimeout(function () {
    divName.removeChild(messageElement);
  }, setTimeoutTime);
}


 // Call the Rust function getFiles
// ######################
// ##### BEGIN HERE #####

// ############################
// ##### INPUT FOR UPLOAD #####
// Create a file input element
let fileInput = document.createElement('input');
fileInput.type = 'file';
fileInput.multiple = true; // Allow multiple files to be selected


// #########################
// ##### UPLOAD BUTTON #####
document.getElementById('uploadButton').addEventListener('click', function () {
  console.log('Upload files button clicked');

  // Trigger the file input click event to open the file explorer
  fileInput.click();

  fileInput.addEventListener('change', function () {
    let files = fileInput.files; // This is a FileList object

    // Convert the FileList to an array of file names
    let filesNames = Array.from(files).map(file => file.name);

    console.log('filesNames:', filesNames);

    invoke('uploadFiles', { filesNames }).then((response) => { // Call the Rust function uploadFiles
      console.log(response);
    });
  });
});


// #########################
// ##### SEARCH BUTTON #####
document.getElementById('searchButton').addEventListener('click', function () {
  console.log('Search file to download button clicked');
  document.getElementById('searchForm').style.display = 'block';
});


// #######################
// ##### SEARCH FORM #####
document.getElementById('submitSearchForm').addEventListener('click', function (event) {
  // Prevent the form from submitting normally
  event.preventDefault();

  console.log('Search form submitted');

  // Get the input values
  var fileNameSubmitted = document.getElementById('fileNameSearchForm').value;
  var fileSizeSubmitted = document.getElementById('fileSizeSearchForm').value;

  console.log('File name submitted: ' + fileNameSubmitted);
  console.log('File size submitted: ' + fileSizeSubmitted);

  // Hide the form
  document.getElementById('searchForm').style.display = 'none';

  // Display a message
  let tmpStr = 'Search form submitted, File Name: ' + fileNameSubmitted + ', File Size: ' + fileSizeSubmitted;
  addMessage(tmpStr, document.getElementById('actions'), 5000);

  // Remove existing search results
  var existingResultsTable = document.getElementById('resultsTableDiv');
  if (existingResultsTable) {
    document.getElementById('actions').removeChild(existingResultsTable);
  }

  // Example results - real results should be taken from the backend
  var results = [
    { name: 'file1.txt', size: 100 },
    { name: 'file2.txt', size: 200 },
    { name: 'file3.txt', size: 300 }
  ];

  

  
  // Create a table
  var tableDiv = document.createElement('div');
  tableDiv.id = 'resultsTableDiv';
  tableDiv.classList.add('clickable');
  addMessage('Files found - Click on file to download', tableDiv, 0, 'tableTitle');
  var table = document.createElement('table');

  // Add table header
  var thead = document.createElement('thead');
  var headerRow = document.createElement('tr');
  ['File Name', 'File Size'].forEach(function (header) {
    var th = document.createElement('th');
    th.textContent = header;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Add table body
  var tbody = document.createElement('tbody');
  results.forEach(function (file) {
    var row = document.createElement('tr');
    [file.name, file.size].forEach(function (cell) {
      var td = document.createElement('td');
      td.textContent = cell;
      row.appendChild(td);
    });
    tbody.appendChild(row);

    // Add an event listener to the row
    row.addEventListener('click', function () {
      console.log('File clicked:', file.name);
      addMessage('Started downloading file: ' + file.name, document.getElementById('actions'), 5000);
    });
  });
  table.appendChild(tbody);
  tableDiv.appendChild(table);

  document.getElementById('actions').appendChild(tableDiv);

  let messageElement = document.getElementById('tableTitle');
  messageElement.style.marginTop = '0';
});


// #####################
// ##### DASHBOARD #####
var refreshRate = 1;  // Refresh rate in seconds
async function populateDashboard() {

  function parseDataString(inputString) {
  const data = [];
  if (inputString == "") {
    return []; 
  }
  const items = inputString.split('|').slice(0,-1);
  for (const item of items) {
    const values = item.split('#');
    const obj = {
      name: values[0],
      downloadPercentage: values[1],
      peersNumber: values[2],
      leechingStatus: values[3] === '1' ? 'Seeding' : 'Leeching'
    };
    data.push(obj);
  }
  return data;
  }

  async function getData() {
  var data = [];
    await invoke("get_files_data").then(result =>{
      data = parseDataString(result);
      console.log(data);
      return data;
    });
    return data;
  }

  var data =  await getData();
  console.log(data);
  // Example data - real data should be taken from the backend
  // var data = [
  //   { name: 'file1.txt', downloadPercentage: 'Downloading... 50%', numberOfPeers: '13', leechingStatus: 'Leeching...' },
  //   { name: 'file2.txt', downloadPercentage: 'Downloading... 75%', numberOfPeers: '5', leechingStatus: 'Leeching...' },
  //   { name: 'file3.txt', downloadPercentage: 'Downloading... 15%', numberOfPeers: '8', leechingStatus: 'Not Leeching' }
  // ];

  // Create a table
  var table = document.createElement('table');

  // Add table header
  var thead = document.createElement('thead');
  var headerRow = document.createElement('tr');
  ['File Name', 'Download Percentage (%)',"Connected Peers", 'Status'].forEach(function (header) {
    var th = document.createElement('th');
    th.textContent = header;
    headerRow.appendChild(th);
  });
  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Add table body
  var tbody = document.createElement('tbody');
  data.forEach(function (file) {
    var row = document.createElement('tr');
    [file.name, file.downloadPercentage, file.peersNumber, file.leechingStatus].forEach(function (cell) {
      var td = document.createElement('td');
      td.textContent = cell;
      row.appendChild(td);
    });
    tbody.appendChild(row);
  });
  table.appendChild(tbody);

  // Add the table to the div
  var dashboardDiv = document.getElementById('appDashboard');
  dashboardDiv.innerHTML = '';  // Clear the div
  dashboardDiv.appendChild(table);

  // Add a timestamp
  addMessage('Updating every ' + refreshRate + ' seconds. Last updated: ' + new Date().toLocaleString(), dashboardDiv, 0, 'timestampDashboard');
  let messageElement = document.getElementById('timestampDashboard');
  messageElement.style.fontSize = '0.8em';
  messageElement.style.marginBottom = '0';

}
// Update the dashboard every second
setInterval(populateDashboard, refreshRate * 1000);
