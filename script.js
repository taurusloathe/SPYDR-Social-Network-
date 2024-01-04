function openTab(tabName) {
  // Hide all tabs
  const tabs = document.querySelectorAll('.tab-content');
  tabs.forEach(tab => tab.style.display = 'none');

  // Show the selected tab
  document.getElementById(tabName).style.display = 'block';
}
