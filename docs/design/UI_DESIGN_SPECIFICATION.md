# Oxidized Wallet - UI Design Specification

## Overview
This document outlines the user interface design for Oxidized Wallet, a local-first personal finance tracking application with double-entry bookkeeping, designed specifically for French users managing EUR accounts.

## Design Principles

### Core Requirements
1. **Clean and Professional** - Inspire trust for financial management
2. **Efficient Data Entry** - Quick transaction input is critical
3. **Clear Hierarchy** - Account tree structure must be intuitive
4. **French Conventions** - Number formatting (1 234,56 €), date format (DD/MM/YYYY)
5. **Responsive** - Works well on different screen sizes
6. **Accessible** - Good contrast, keyboard navigation

### Visual Design
- Modern but not trendy - a tool for daily use over years
- Clean white/gray color scheme
- Subtle shadows and hover effects
- No unnecessary animations
- Focus on data clarity

## Application Structure

### Layout Architecture
```
MainApp
├── Sidebar (Collapsible)
│   ├── Header
│   │   ├── Logo (OW)
│   │   ├── App Name
│   │   └── Toggle Button
│   ├── Navigation Menu
│   │   ├── Tableau de bord (Dashboard)
│   │   ├── Comptes (Accounts)
│   │   ├── Transactions
│   │   └── Rapports (Reports)
│   ├── Account Tree
│   │   ├── Section Title ("COMPTES")
│   │   └── Bank List (with total balances only)
│   │       ├── BoursoBank (15,234.56 €)
│   │       │   ├── Compte Courant
│   │       │   └── Livret A
│   │       └── Crédit Agricole (8,450.00 €)
│   └── Quick Action Button
│       └── "Nouvelle transaction"
│
├── Main Content Area
│   ├── Top Bar
│   │   ├── Page Title (Dynamic)
│   │   ├── Sync Status
│   │   └── Settings Button
│   │
│   └── Content Views (Only one visible at a time)
│       ├── Dashboard View
│       ├── Transactions View
│       ├── Accounts View
│       ├── Budget View
│       └── Report View
│
└── Modals
    └── Transaction Entry Modal
```

## View Specifications

### 1. Dashboard View
**Purpose**: Provide at-a-glance financial overview

**Components**:
- **Overview Cards** (4 metrics grid)
  - Valeur nette (Net Worth) - with trend indicator
  - Actifs totaux (Total Assets) - with account count
  - Revenus du mois (Monthly Income)
  - Dépenses du mois (Monthly Expenses) - with comparison
- **Recent Transactions Widget**
  - Last 3 transactions with visual indicators
  - Color coding: Green (income), Red (expenses), Blue (transfers)
  - "View all" link to full transaction list

**Future Enhancement**: Net worth evolution graph

### 2. Transaction Entry Modal
**Purpose**: Efficient double-entry transaction recording

**Components**:
- **Header Section**
  - Date picker (DD/MM/YYYY format)
  - Description field
- **Transaction Type Toggle**
  - Revenu (Income)
  - Dépense (Expense)
  - Transfert (Transfer)
- **Amount Input**
  - Large numeric input with € symbol
  - French number formatting
- **Double Entry Section**
  - Credit entry with account selector
  - Debit entry with account selector
  - Real-time balance validation indicator
- **Action Buttons**
  - Cancel / Save

**Key Features**:
- Always accessible via sidebar button
- Keyboard shortcut support
- Smart account pre-selection based on transaction type

### 3. Accounts Management View
**Purpose**: Organize bank accounts and categories

**Components**:
- **Bank Cards Grid**
  - Visual bank cards showing:
    - Bank name and icon
    - Total balance
    - Sub-account list with balances
    - "Add sub-account" action
  - "Add new bank" card
- **Category Management Section**
  - Income categories
  - Expense categories with color-coded icons
  - Add category action

### 4. Transaction History View
**Purpose**: View and manage all transactions

**Components**:
- **Filter Bar**
  - Period selector (dropdown)
  - Account filter (dropdown)
  - Transaction type filter
  - Search input
- **Transaction Table**
  - Columns: Date, Description, Debit Account, Credit Account, Amount
  - Row actions: Edit, Delete, Duplicate
  - Pagination controls
- **Bulk Actions** (future)

### 5. Monthly Report View
**Purpose**: Analyze monthly financial performance

**Components**:
- **Month Selector** (dropdown)
- **Summary Cards**
  - Total Income
  - Total Expenses
  - Monthly Savings (amount & percentage)
- **Expense Breakdown**
  - Category list with:
    - Amount and percentage
    - Visual progress bars
    - Color coding by category

### 6. Budget Tracking View (Future Phase)
**Purpose**: Monitor spending against budgets

**Components**:
- **Budget Overview Cards**
  - Total Budget
  - Planned Savings
  - Budget Alerts
- **Category Budget List**
  - Progress bars with handle indicators
  - Color coding: Green (on track), Orange (warning), Red (exceeded)
  - Remaining amount display
- **Recommendations Section**

## UI Component Library

### Layout Components
- **Sidebar** - Collapsible navigation panel
- **TopBar** - Application header with title and actions
- **MainLayout** - Container for sidebar + content
- **PageContainer** - Content area wrapper

### Navigation Components
- **NavMenu** - Vertical navigation list
- **NavItem** - Individual navigation link with active state
- **AccountTree** - Hierarchical account display
- **TreeItem** - Expandable/collapsible tree node

### Card Components
- **MetricCard** - Dashboard overview cards (icon, label, value, trend)
- **SummaryCard** - Bordered cards for summaries
- **AccountCard** - Bank/account display cards with hover effects

### Form Components
- **TextInput** - Basic text input field
- **DateInput** - Date picker (DD/MM/YYYY)
- **AmountInput** - Currency-formatted input (1 234,56 €)
- **Select/Dropdown** - Selection dropdowns
- **Button** - Primary, secondary, and ghost variants
- **IconButton** - Button with just an icon
- **ToggleGroup** - Transaction type selector

### Data Display Components
- **DataTable** - Sortable transaction table
- **TablePagination** - Pagination controls
- **ProgressBar** - Budget/category progress indicator
- **Badge** - Status indicators
- **Stat** - Number with label display

### Modal/Overlay Components
- **Modal** - Overlay dialog container
- **ModalHeader** - Modal title bar
- **ModalFooter** - Modal action buttons

### List Components
- **TransactionItem** - Individual transaction display
- **CategoryItem** - Category list item with icon
- **AccountListItem** - Account in management view

### Utility Components
- **Icon** - Wrapper for Lucide icons
- **Divider** - Horizontal separator
- **EmptyState** - No data placeholder
- **LoadingSpinner** - Loading indicator
- **Tooltip** - Hover information

### Chart Components (Future)
- **LineChart** - Net worth evolution
- **PieChart** - Expense breakdown
- **BarChart** - Monthly comparisons

### Specialized Components
- **DoubleEntryForm** - Credit/Debit entry component
- **BalanceIndicator** - Shows if entries balance
- **FilterBar** - Combined filter controls
- **BudgetProgressItem** - Budget category with progress

## Color Scheme

### Primary Colors
- **Blue** (`#3B82F6`) - Primary actions, links
- **Green** (`#10B981`) - Income, positive values
- **Red** (`#EF4444`) - Expenses, negative values

### Semantic Colors
- **Purple** - Housing/Logement category
- **Orange** - Food/Alimentation category
- **Pink** - Health/Santé category
- **Green** - Transport category

### Neutral Colors
- **Gray Scale** - UI elements, text, borders
- **White** - Card backgrounds, modals
- **Gray-50** - Page background

## Icon System
Using Lucide icons throughout:
- **Navigation**: Home, Wallet, ArrowLeftRight, BarChart3
- **Actions**: Plus, Edit, Trash, Copy
- **Categories**: ShoppingCart, Home, Car, Heart
- **Banks**: Building, CreditCard, PiggyBank
- **Indicators**: TrendingUp, TrendingDown, Check

## Interaction Patterns

### Navigation
- Collapsible sidebar preserves state
- Active navigation item highlighted
- Keyboard navigation support (Tab, Arrow keys)

### Data Entry
- Transaction modal accessible via:
  - Sidebar button (always visible)
  - Keyboard shortcut (Ctrl+N)
  - Context menus (future)
- Form validation on submit
- Real-time balance validation for transactions

### Feedback
- Hover states on all interactive elements
- Loading states for async operations
- Success/error toast notifications (future)
- Confirmation dialogs for destructive actions

## Responsive Behavior
- **Desktop** (>1200px): Full sidebar, multi-column layouts
- **Tablet** (768-1200px): Collapsible sidebar, adjusted grids
- **Mobile** (future): Bottom navigation, single column

## Accessibility
- ARIA labels on all interactive elements
- Keyboard navigation throughout
- High contrast mode support
- Screen reader friendly structure
- Focus indicators on all inputs

## Future Enhancements
1. **Dark Mode** - Already supported via theme system
2. **Customizable Dashboard** - Widget arrangement
3. **Data Visualizations** - Charts and graphs
4. **Quick Filters** - Saved filter presets
5. **Bulk Operations** - Multi-select actions
6. **Drag & Drop** - Account reorganization
7. **Export Options** - PDF reports, CSV data

## Technical Implementation Notes
- Built with Vue 3 Composition API
- shadcn-vue (reka-ui) component library
- Tailwind CSS for styling
- TypeScript for type safety
- Tauri for desktop integration

This design specification provides a comprehensive blueprint for implementing the Oxidized Wallet UI, ensuring consistency, usability, and extensibility for future features.