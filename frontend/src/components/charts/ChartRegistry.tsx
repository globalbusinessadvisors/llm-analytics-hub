/**
 * Chart Registry
 * Central registry for all 50+ chart types with lazy loading
 */

import { lazy, ComponentType } from 'react';
import { ChartType } from '@/types/dashboard';

export interface ChartComponentProps {
  data: unknown;
  config?: Record<string, unknown>;
  width?: number;
  height?: number;
  onDataPointClick?: (data: unknown) => void;
  isRealtime?: boolean;
}

// Chart component registry with lazy loading
const chartComponents: Record<ChartType, ComponentType<ChartComponentProps>> = {
  // Line charts
  [ChartType.Line]: lazy(() => import('./LineChart')) as ComponentType<ChartComponentProps>,
  [ChartType.SmoothLine]: lazy(() => import('./SmoothLineChart')),
  [ChartType.SteppedLine]: lazy(() => import('./SteppedLineChart')),
  [ChartType.MultiLine]: lazy(() => import('./MultiLineChart')),
  [ChartType.StackedLine]: lazy(() => import('./StackedLineChart')),
  [ChartType.AreaLine]: lazy(() => import('./AreaLineChart')),

  // Bar charts
  [ChartType.Bar]: lazy(() => import('./BarChart')),
  [ChartType.HorizontalBar]: lazy(() => import('./HorizontalBarChart')),
  [ChartType.StackedBar]: lazy(() => import('./StackedBarChart')),
  [ChartType.GroupedBar]: lazy(() => import('./GroupedBarChart')),

  // Pie/Donut charts
  [ChartType.Pie]: lazy(() => import('./PieChart')),
  [ChartType.Donut]: lazy(() => import('./DonutChart')),
  [ChartType.SemiDonut]: lazy(() => import('./SemiDonutChart')),

  // Scatter/Bubble
  [ChartType.Scatter]: lazy(() => import('./ScatterChart')),
  [ChartType.Bubble]: lazy(() => import('./BubbleChart')),

  // Heatmaps
  [ChartType.Heatmap]: lazy(() => import('./Heatmap')) as ComponentType<ChartComponentProps>,
  [ChartType.CalendarHeatmap]: lazy(() => import('./CalendarHeatmap')),

  // Specialized charts
  [ChartType.Sankey]: lazy(() => import('./SankeyDiagram')) as ComponentType<ChartComponentProps>,
  [ChartType.Treemap]: lazy(() => import('./TreemapChart')),
  [ChartType.Sunburst]: lazy(() => import('./SunburstChart')),
  [ChartType.Funnel]: lazy(() => import('./FunnelChart')),
  [ChartType.Gauge]: lazy(() => import('./GaugeChart')),
  [ChartType.Radar]: lazy(() => import('./RadarChart')),
  [ChartType.Polar]: lazy(() => import('./PolarChart')),

  // Time-series specific
  [ChartType.TimeSeriesLine]: lazy(() => import('./TimeSeriesLineChart')) as ComponentType<ChartComponentProps>,
  [ChartType.TimeSeriesArea]: lazy(() => import('./TimeSeriesAreaChart')),
  [ChartType.Candlestick]: lazy(() => import('./CandlestickChart')),

  // Network/Graph
  [ChartType.ForceDirectedGraph]: lazy(() => import('./ForceDirectedGraph')),
  [ChartType.ChordDiagram]: lazy(() => import('./ChordDiagram')),

  // Statistical
  [ChartType.BoxPlot]: lazy(() => import('./BoxPlot')),
  [ChartType.Violin]: lazy(() => import('./ViolinPlot')),
  [ChartType.Histogram]: lazy(() => import('./HistogramChart')),

  // Comparison
  [ChartType.BulletChart]: lazy(() => import('./BulletChart')),
  [ChartType.WaterfallChart]: lazy(() => import('./WaterfallChart')),

  // Geographic
  [ChartType.Choropleth]: lazy(() => import('./ChoroplethMap')),
  [ChartType.DotMap]: lazy(() => import('./DotMap')),

  // Tables
  [ChartType.DataTable]: lazy(() => import('./DataTable')),
  [ChartType.PivotTable]: lazy(() => import('./PivotTable')),

  // Single value displays
  [ChartType.SingleValue]: lazy(() => import('./SingleValue')),
  [ChartType.SingleValueWithTrend]: lazy(() => import('./SingleValueTrend')),
  [ChartType.StatusIndicator]: lazy(() => import('./StatusIndicator')),
  [ChartType.ProgressBar]: lazy(() => import('./ProgressBar')),

  // Custom composites
  [ChartType.SparkLine]: lazy(() => import('./SparkLine')),
  [ChartType.MiniChart]: lazy(() => import('./MiniChart')),
  [ChartType.ComparisonCard]: lazy(() => import('./ComparisonCard')),
};

export function getChartComponent(type: ChartType): ComponentType<ChartComponentProps> | null {
  return chartComponents[type] || null;
}

export function getAvailableChartTypes(): ChartType[] {
  return Object.keys(chartComponents) as ChartType[];
}

// Chart metadata for UI display
export interface ChartMetadata {
  type: ChartType;
  name: string;
  description: string;
  category: ChartCategory;
  icon: string;
  preview?: string;
  requiredDataFields: string[];
  supportedDataTypes: DataType[];
  useCases: string[];
}

export enum ChartCategory {
  TimeSeries = 'Time Series',
  Comparison = 'Comparison',
  Distribution = 'Distribution',
  Composition = 'Composition',
  Relationship = 'Relationship',
  Geographic = 'Geographic',
  Statistical = 'Statistical',
  Tables = 'Tables',
  Indicators = 'Indicators',
}

export enum DataType {
  Numeric = 'numeric',
  Categorical = 'categorical',
  Temporal = 'temporal',
  Geospatial = 'geospatial',
  Hierarchical = 'hierarchical',
  Network = 'network',
}

export const chartMetadata: Record<ChartType, ChartMetadata> = {
  [ChartType.Line]: {
    type: ChartType.Line,
    name: 'Line Chart',
    description: 'Display trends over time with connected data points',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Trend analysis', 'Performance monitoring', 'Time-series data'],
  },
  [ChartType.Bar]: {
    type: ChartType.Bar,
    name: 'Bar Chart',
    description: 'Compare values across categories with vertical bars',
    category: ChartCategory.Comparison,
    icon: 'BarChart3',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Category comparison', 'Ranking', 'Distribution'],
  },
  [ChartType.Heatmap]: {
    type: ChartType.Heatmap,
    name: 'Heatmap',
    description: 'Visualize matrix data with color-coded cells',
    category: ChartCategory.Distribution,
    icon: 'Grid',
    requiredDataFields: ['x', 'y', 'value'],
    supportedDataTypes: [DataType.Numeric, DataType.Categorical],
    useCases: ['Correlation analysis', 'Time patterns', 'Intensity mapping'],
  },
  [ChartType.Sankey]: {
    type: ChartType.Sankey,
    name: 'Sankey Diagram',
    description: 'Show flow and relationships between entities',
    category: ChartCategory.Relationship,
    icon: 'Network',
    requiredDataFields: ['source', 'target', 'value'],
    supportedDataTypes: [DataType.Network, DataType.Numeric],
    useCases: ['Flow analysis', 'Resource allocation', 'Process visualization'],
  },
  [ChartType.Gauge]: {
    type: ChartType.Gauge,
    name: 'Gauge Chart',
    description: 'Display single metric with target ranges',
    category: ChartCategory.Indicators,
    icon: 'Gauge',
    requiredDataFields: ['value'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['KPI monitoring', 'Performance indicators', 'Progress tracking'],
  },
  [ChartType.SmoothLine]: {
    type: ChartType.SmoothLine,
    name: 'Smooth Line Chart',
    description: 'Line chart with smooth curves',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Trend analysis'],
  },
  [ChartType.SteppedLine]: {
    type: ChartType.SteppedLine,
    name: 'Stepped Line Chart',
    description: 'Line chart with step interpolation',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Step function data'],
  },
  [ChartType.MultiLine]: {
    type: ChartType.MultiLine,
    name: 'Multi Line Chart',
    description: 'Multiple line series',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Multiple series comparison'],
  },
  [ChartType.StackedLine]: {
    type: ChartType.StackedLine,
    name: 'Stacked Line Chart',
    description: 'Stacked line series',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Cumulative trends'],
  },
  [ChartType.AreaLine]: {
    type: ChartType.AreaLine,
    name: 'Area Line Chart',
    description: 'Line chart with filled area',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric, DataType.Temporal],
    useCases: ['Volume visualization'],
  },
  [ChartType.HorizontalBar]: {
    type: ChartType.HorizontalBar,
    name: 'Horizontal Bar Chart',
    description: 'Bar chart with horizontal bars',
    category: ChartCategory.Comparison,
    icon: 'BarChart3',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Category comparison'],
  },
  [ChartType.StackedBar]: {
    type: ChartType.StackedBar,
    name: 'Stacked Bar Chart',
    description: 'Stacked bar chart',
    category: ChartCategory.Comparison,
    icon: 'BarChart3',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Part-to-whole comparison'],
  },
  [ChartType.GroupedBar]: {
    type: ChartType.GroupedBar,
    name: 'Grouped Bar Chart',
    description: 'Grouped bar chart',
    category: ChartCategory.Comparison,
    icon: 'BarChart3',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Group comparison'],
  },
  [ChartType.Pie]: {
    type: ChartType.Pie,
    name: 'Pie Chart',
    description: 'Circular pie chart',
    category: ChartCategory.Composition,
    icon: 'PieChart',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Composition analysis'],
  },
  [ChartType.Donut]: {
    type: ChartType.Donut,
    name: 'Donut Chart',
    description: 'Donut chart with center hole',
    category: ChartCategory.Composition,
    icon: 'PieChart',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Composition analysis'],
  },
  [ChartType.SemiDonut]: {
    type: ChartType.SemiDonut,
    name: 'Semi Donut Chart',
    description: 'Half donut chart',
    category: ChartCategory.Composition,
    icon: 'PieChart',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Gauge-like visualization'],
  },
  [ChartType.Scatter]: {
    type: ChartType.Scatter,
    name: 'Scatter Chart',
    description: 'Scatter plot',
    category: ChartCategory.Relationship,
    icon: 'Scatter',
    requiredDataFields: ['x', 'y'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Correlation analysis'],
  },
  [ChartType.Bubble]: {
    type: ChartType.Bubble,
    name: 'Bubble Chart',
    description: 'Bubble chart with size dimension',
    category: ChartCategory.Relationship,
    icon: 'Scatter',
    requiredDataFields: ['x', 'y', 'size'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Multi-dimensional comparison'],
  },
  [ChartType.CalendarHeatmap]: {
    type: ChartType.CalendarHeatmap,
    name: 'Calendar Heatmap',
    description: 'Calendar-based heatmap',
    category: ChartCategory.Distribution,
    icon: 'Calendar',
    requiredDataFields: ['date', 'value'],
    supportedDataTypes: [DataType.Temporal, DataType.Numeric],
    useCases: ['Time-based patterns'],
  },
  [ChartType.Treemap]: {
    type: ChartType.Treemap,
    name: 'Treemap',
    description: 'Hierarchical treemap',
    category: ChartCategory.Composition,
    icon: 'Grid',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Hierarchical, DataType.Numeric],
    useCases: ['Hierarchical composition'],
  },
  [ChartType.Sunburst]: {
    type: ChartType.Sunburst,
    name: 'Sunburst Chart',
    description: 'Radial hierarchical chart',
    category: ChartCategory.Composition,
    icon: 'Circle',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Hierarchical, DataType.Numeric],
    useCases: ['Hierarchical composition'],
  },
  [ChartType.Funnel]: {
    type: ChartType.Funnel,
    name: 'Funnel Chart',
    description: 'Funnel visualization',
    category: ChartCategory.Comparison,
    icon: 'Filter',
    requiredDataFields: ['stage', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Conversion analysis'],
  },
  [ChartType.Radar]: {
    type: ChartType.Radar,
    name: 'Radar Chart',
    description: 'Radar/spider chart',
    category: ChartCategory.Comparison,
    icon: 'Radar',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Multi-dimensional comparison'],
  },
  [ChartType.Polar]: {
    type: ChartType.Polar,
    name: 'Polar Chart',
    description: 'Polar area chart',
    category: ChartCategory.Comparison,
    icon: 'Circle',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Cyclic data'],
  },
  [ChartType.TimeSeriesLine]: {
    type: ChartType.TimeSeriesLine,
    name: 'Time Series Line',
    description: 'Advanced time series line chart',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['timestamp', 'value'],
    supportedDataTypes: [DataType.Temporal, DataType.Numeric],
    useCases: ['Time series analysis'],
  },
  [ChartType.TimeSeriesArea]: {
    type: ChartType.TimeSeriesArea,
    name: 'Time Series Area',
    description: 'Time series area chart',
    category: ChartCategory.TimeSeries,
    icon: 'TrendingUp',
    requiredDataFields: ['timestamp', 'value'],
    supportedDataTypes: [DataType.Temporal, DataType.Numeric],
    useCases: ['Time series volume'],
  },
  [ChartType.Candlestick]: {
    type: ChartType.Candlestick,
    name: 'Candlestick Chart',
    description: 'Candlestick chart for OHLC data',
    category: ChartCategory.TimeSeries,
    icon: 'Candle',
    requiredDataFields: ['timestamp', 'open', 'high', 'low', 'close'],
    supportedDataTypes: [DataType.Temporal, DataType.Numeric],
    useCases: ['Financial data'],
  },
  [ChartType.ForceDirectedGraph]: {
    type: ChartType.ForceDirectedGraph,
    name: 'Force Directed Graph',
    description: 'Network graph with force simulation',
    category: ChartCategory.Relationship,
    icon: 'Network',
    requiredDataFields: ['nodes', 'links'],
    supportedDataTypes: [DataType.Network],
    useCases: ['Network analysis'],
  },
  [ChartType.ChordDiagram]: {
    type: ChartType.ChordDiagram,
    name: 'Chord Diagram',
    description: 'Circular relationship diagram',
    category: ChartCategory.Relationship,
    icon: 'Circle',
    requiredDataFields: ['source', 'target', 'value'],
    supportedDataTypes: [DataType.Network, DataType.Numeric],
    useCases: ['Flow between entities'],
  },
  [ChartType.BoxPlot]: {
    type: ChartType.BoxPlot,
    name: 'Box Plot',
    description: 'Statistical box plot',
    category: ChartCategory.Statistical,
    icon: 'Box',
    requiredDataFields: ['category', 'values'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Distribution analysis'],
  },
  [ChartType.Violin]: {
    type: ChartType.Violin,
    name: 'Violin Plot',
    description: 'Violin plot for distribution',
    category: ChartCategory.Statistical,
    icon: 'Box',
    requiredDataFields: ['category', 'values'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Distribution analysis'],
  },
  [ChartType.Histogram]: {
    type: ChartType.Histogram,
    name: 'Histogram',
    description: 'Frequency distribution',
    category: ChartCategory.Statistical,
    icon: 'BarChart3',
    requiredDataFields: ['value'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Distribution analysis'],
  },
  [ChartType.BulletChart]: {
    type: ChartType.BulletChart,
    name: 'Bullet Chart',
    description: 'Performance vs target',
    category: ChartCategory.Comparison,
    icon: 'Target',
    requiredDataFields: ['value', 'target'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Goal tracking'],
  },
  [ChartType.WaterfallChart]: {
    type: ChartType.WaterfallChart,
    name: 'Waterfall Chart',
    description: 'Cumulative change visualization',
    category: ChartCategory.Comparison,
    icon: 'TrendingUp',
    requiredDataFields: ['category', 'value'],
    supportedDataTypes: [DataType.Categorical, DataType.Numeric],
    useCases: ['Sequential changes'],
  },
  [ChartType.Choropleth]: {
    type: ChartType.Choropleth,
    name: 'Choropleth Map',
    description: 'Geographic regions with color coding',
    category: ChartCategory.Geographic,
    icon: 'Map',
    requiredDataFields: ['region', 'value'],
    supportedDataTypes: [DataType.Geospatial, DataType.Numeric],
    useCases: ['Geographic distribution'],
  },
  [ChartType.DotMap]: {
    type: ChartType.DotMap,
    name: 'Dot Map',
    description: 'Point-based geographic map',
    category: ChartCategory.Geographic,
    icon: 'Map',
    requiredDataFields: ['lat', 'lng'],
    supportedDataTypes: [DataType.Geospatial],
    useCases: ['Point distribution'],
  },
  [ChartType.DataTable]: {
    type: ChartType.DataTable,
    name: 'Data Table',
    description: 'Display data in tabular format with sorting and filtering',
    category: ChartCategory.Tables,
    icon: 'Table',
    requiredDataFields: [],
    supportedDataTypes: [DataType.Numeric, DataType.Categorical, DataType.Temporal],
    useCases: ['Detailed data view', 'Data exploration', 'Export preparation'],
  },
  [ChartType.PivotTable]: {
    type: ChartType.PivotTable,
    name: 'Pivot Table',
    description: 'Interactive pivot table',
    category: ChartCategory.Tables,
    icon: 'Table',
    requiredDataFields: [],
    supportedDataTypes: [DataType.Numeric, DataType.Categorical],
    useCases: ['Data aggregation'],
  },
  [ChartType.SingleValue]: {
    type: ChartType.SingleValue,
    name: 'Single Value',
    description: 'Display single metric value',
    category: ChartCategory.Indicators,
    icon: 'Hash',
    requiredDataFields: ['value'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['KPI display'],
  },
  [ChartType.SingleValueWithTrend]: {
    type: ChartType.SingleValueWithTrend,
    name: 'Single Value with Trend',
    description: 'Value with trend indicator',
    category: ChartCategory.Indicators,
    icon: 'TrendingUp',
    requiredDataFields: ['value', 'change'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['KPI with trend'],
  },
  [ChartType.StatusIndicator]: {
    type: ChartType.StatusIndicator,
    name: 'Status Indicator',
    description: 'Status with color coding',
    category: ChartCategory.Indicators,
    icon: 'Circle',
    requiredDataFields: ['status'],
    supportedDataTypes: [DataType.Categorical],
    useCases: ['System status'],
  },
  [ChartType.ProgressBar]: {
    type: ChartType.ProgressBar,
    name: 'Progress Bar',
    description: 'Progress visualization',
    category: ChartCategory.Indicators,
    icon: 'Progress',
    requiredDataFields: ['value', 'max'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Progress tracking'],
  },
  [ChartType.SparkLine]: {
    type: ChartType.SparkLine,
    name: 'Spark Line',
    description: 'Compact inline trend',
    category: ChartCategory.Indicators,
    icon: 'TrendingUp',
    requiredDataFields: ['values'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Inline trends'],
  },
  [ChartType.MiniChart]: {
    type: ChartType.MiniChart,
    name: 'Mini Chart',
    description: 'Compact visualization',
    category: ChartCategory.Indicators,
    icon: 'Chart',
    requiredDataFields: ['values'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['Compact display'],
  },
  [ChartType.ComparisonCard]: {
    type: ChartType.ComparisonCard,
    name: 'Comparison Card',
    description: 'Side-by-side comparison',
    category: ChartCategory.Comparison,
    icon: 'Compare',
    requiredDataFields: ['values'],
    supportedDataTypes: [DataType.Numeric],
    useCases: ['A/B comparison'],
  },
};

export function getChartMetadata(type: ChartType): ChartMetadata | null {
  return chartMetadata[type] || null;
}

export function getChartsByCategory(category: ChartCategory): ChartMetadata[] {
  return Object.values(chartMetadata).filter((meta) => meta.category === category);
}
