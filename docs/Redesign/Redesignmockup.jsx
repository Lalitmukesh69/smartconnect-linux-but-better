import React, { useState, useEffect, useRef, useMemo } from 'react';
import {
    Smartphone,
    LayoutGrid,
    Monitor,
    Folder,
    Bell,
    Battery,
    Wifi,
    Signal,
    Bluetooth,
    Info,
    Pencil,
    Check,
    ChevronRight,
    ChevronDown,
    Search,
    RotateCw,
    MousePointer,
    Copy,
    FolderOpen,
    Sparkles,
    Command,
    X,
    ArrowUpRight,
    RefreshCw,
    HardDrive,
    Send,
    Sliders,
    List,
    Grid,
    Terminal,
    Cpu,
    Shield,
    Radio,
    CornerDownLeft,
    Layers,
    Maximize2,
    Lock,
    Eye,
    Clock,
    Activity
} from 'lucide-react';

// ============================================================================
// DATASET: ALL APPS FROM THE MVP SCREENSHOT (CURATED FOR MONOCHROME STUDIO)
// ============================================================================
const ALL_APPS_DATA = [
    { id: 'echomusic', name: 'Echo Music', category: 'Media', glyph: '((◉))', monoTone: 'from-[#2A2A32] to-[#121216]', studioTone: 'from-[#1E293B] to-[#0F172A]', recent: true, version: '4.2.1', latency: '0.8ms' },
    { id: 'airbnb', name: 'Airbnb', category: 'Productivity', glyph: 'A', monoTone: 'from-[#27272C] to-[#141417]', studioTone: 'from-[#2D1B22] to-[#171114]', version: '24.18' },
    { id: 'amazingtalker', name: 'AmazingTalker', category: 'Productivity', glyph: 'AT', monoTone: 'from-[#E4E4E7] to-[#A1A1AA]', lightTile: true, studioTone: 'from-[#E4E4E7] to-[#CBD5E1]', version: '5.1.0' },
    { id: 'amazon', name: 'Amazon', category: 'Productivity', glyph: 'a➔', monoTone: 'from-[#232329] to-[#101014]', studioTone: 'from-[#1E242B] to-[#0F1318]', version: '28.9.0' },
    { id: 'amazonpay', name: 'Amazon Pay', category: 'Productivity', glyph: 'PAY', monoTone: 'from-[#26262C] to-[#121216]', studioTone: 'from-[#242B35] to-[#12161C]', version: '3.0.4' },
    { id: 'authenticator', name: 'Authenticator', category: 'System', glyph: '✱', monoTone: 'from-[#2C2C34] to-[#141418]', studioTone: 'from-[#1E2530] to-[#10141B]', version: '6.8.2' },
    { id: 'b612', name: 'B612', category: 'Media', glyph: 'B612', monoTone: 'from-[#25252B] to-[#121215]', studioTone: 'from-[#182C2E] to-[#0E181A]', version: '12.4.1' },
    { id: 'bgmi', name: 'Battlegrounds India', category: 'Media', glyph: 'BGMI', monoTone: 'from-[#2E2E36] to-[#141417]', studioTone: 'from-[#2B2518] to-[#16130D]', version: '3.1.0' },
    { id: 'blip', name: 'Blip', category: 'System', glyph: 'B:', monoTone: 'from-[#292930] to-[#131317]', studioTone: 'from-[#251D33] to-[#130F1C]', version: '1.9.4' },
    { id: 'bluesky', name: 'Bluesky', category: 'Productivity', glyph: '🦋', monoTone: 'from-[#28282F] to-[#121216]', studioTone: 'from-[#17253B] to-[#0E1522]', version: '1.83' },
    { id: 'brave', name: 'Brave', category: 'AI & Dev', glyph: 'BRV', monoTone: 'from-[#2A2A31] to-[#131317]', studioTone: 'from-[#2E1E19] to-[#17100E]', version: '1.66' },
    { id: 'buttonmapper', name: 'Button Mapper', category: 'System', glyph: '⌘', monoTone: 'from-[#27272E] to-[#121216]', studioTone: 'from-[#1B2638] to-[#0F1520]', version: '3.09' },
    { id: 'calculator', name: 'Calculator', category: 'System', glyph: '±=', monoTone: 'from-[#2E2E35] to-[#151519]', studioTone: 'from-[#192B22] to-[#0E1813]', version: '8.6' },
    { id: 'calendar', name: 'Calendar', category: 'Productivity', glyph: '31', monoTone: 'from-[#F4F4F5] to-[#D4D4D8]', lightTile: true, studioTone: 'from-[#E2E8F0] to-[#CBD5E1]', version: '2026.4' },
    { id: 'camera', name: 'Camera', category: 'Media', glyph: '◉', monoTone: 'from-[#32323A] to-[#16161A]', studioTone: 'from-[#27272A] to-[#18181B]', version: '9.4.1' },
    { id: 'chatgpt', name: 'ChatGPT', category: 'AI & Dev', glyph: '❂', monoTone: 'from-[#2B2B33] to-[#121216]', studioTone: 'from-[#152B26] to-[#0C1815]', recent: true, version: '1.2026' },
    { id: 'chrome', name: 'Chrome', category: 'AI & Dev', glyph: '◎', monoTone: 'from-[#292930] to-[#131317]', studioTone: 'from-[#1B2536] to-[#0F141E]', version: '126.0' },
    { id: 'claude', name: 'Claude', category: 'AI & Dev', glyph: '✳', monoTone: 'from-[#303038] to-[#151519]', studioTone: 'from-[#2E221D] to-[#181210]', recent: true, version: '3.7.0' },
    { id: 'clock', name: 'Clock', category: 'System', glyph: '◷', monoTone: 'from-[#1F1F24] to-[#0D0D10]', studioTone: 'from-[#1C1C21] to-[#0D0D10]', version: '7.6' },
    { id: 'cohesion', name: 'Cohesion', category: 'AI & Dev', glyph: '⚡', monoTone: 'from-[#24242A] to-[#101013]', studioTone: 'from-[#14261D] to-[#0B1410]', version: '2.1.0' },
    { id: 'contacts', name: 'Contacts', category: 'Productivity', glyph: 'ID', monoTone: 'from-[#2A2A32] to-[#131317]', studioTone: 'from-[#1B2638] to-[#0E1520]', version: '4.11' },
    { id: 'devicehelp', name: 'Device Help', category: 'System', glyph: '?', monoTone: 'from-[#28282F] to-[#121216]', studioTone: 'from-[#291F28] to-[#141014]', version: '11.0' },
    { id: 'digipaws', name: 'DigiPaws', category: 'System', glyph: 'II', monoTone: 'from-[#26262C] to-[#111115]', studioTone: 'from-[#222227] to-[#111115]', version: '1.4.2' },
    { id: 'wellbeing', name: 'Digital Wellbeing', category: 'System', glyph: '◇', monoTone: 'from-[#2B2B32] to-[#141418]', studioTone: 'from-[#182921] to-[#0D1712]', version: '1.18' },
    { id: 'discord', name: 'Discord', category: 'Productivity', glyph: 'DSC', monoTone: 'from-[#2A2A33] to-[#141418]', studioTone: 'from-[#1E2038] to-[#111221]', version: '234.1' },
    { id: 'docs', name: 'Docs', category: 'Productivity', glyph: '☰', monoTone: 'from-[#2D2D35] to-[#151519]', studioTone: 'from-[#1B2538] to-[#0E1420]', version: '1.24' },
    { id: 'dodopayments', name: 'Dodo Payments', category: 'Productivity', glyph: 'DD', monoTone: 'from-[#E4E4E7] to-[#A1A1AA]', lightTile: true, studioTone: 'from-[#D9E2EC] to-[#BCCCDC]', version: '2.0.1' },
    { id: 'dolbyatmos', name: 'Dolby Atmos', category: 'Media', glyph: '◖◗', monoTone: 'from-[#222228] to-[#0F0F13]', studioTone: 'from-[#1C1D2B] to-[#0E0F17]', version: '4.0' },
    { id: 'drive', name: 'Drive', category: 'Productivity', glyph: '△', monoTone: 'from-[#2C2C34] to-[#141418]', studioTone: 'from-[#1E292B] to-[#101618]', version: '2.24' },
    { id: 'edits', name: 'Edits', category: 'Media', glyph: 'EDT', monoTone: 'from-[#292930] to-[#121216]', studioTone: 'from-[#2B1D29] to-[#160F15]', version: '1.2.0' },
    { id: 'escapelab', name: 'Escape Lab', category: 'Media', glyph: 'LAB', monoTone: 'from-[#27272E] to-[#121215]', studioTone: 'from-[#29221E] to-[#15110F]', version: '1.0.8' },
    { id: 'etihad', name: 'Etihad', category: 'Productivity', glyph: 'EY', monoTone: 'from-[#2D2D34] to-[#141418]', studioTone: 'from-[#2B261B] to-[#16130E]', version: '6.4.0' },
    { id: 'expogo', name: 'Expo Go', category: 'AI & Dev', glyph: '∧', monoTone: 'from-[#F4F4F5] to-[#D4D4D8]', lightTile: true, studioTone: 'from-[#E4E4E7] to-[#C4C4CC]', version: '51.0.0' },
    { id: 'familyspace', name: 'Family Space', category: 'System', glyph: '◎', monoTone: 'from-[#25252C] to-[#111115]', studioTone: 'from-[#182232] to-[#0D131C]', version: '3.2.1' },
    { id: 'feedback', name: 'Feedback', category: 'System', glyph: 'FDB', monoTone: 'from-[#282830] to-[#131317]', studioTone: 'from-[#1E1E32] to-[#10101C]', version: '2.0' },
    { id: 'ffshare', name: 'FFShare', category: 'System', glyph: '⇄', monoTone: 'from-[#26262E] to-[#121216]', studioTone: 'from-[#16292B] to-[#0C1618]', version: '1.5.3' },
    { id: 'files', name: 'Files', category: 'System', glyph: 'FLD', monoTone: 'from-[#2E2E36] to-[#141418]', studioTone: 'from-[#1A2638] to-[#0E1520]', version: '14.2' },
    { id: 'flashdim', name: 'FlashDim', category: 'System', glyph: '⚡', monoTone: 'from-[#232329] to-[#101013]', studioTone: 'from-[#242429] to-[#111114]', version: '2.3.1' },
    { id: 'flightradar24', name: 'Flightradar24', category: 'Productivity', glyph: 'FR24', monoTone: 'from-[#292931] to-[#131317]', studioTone: 'from-[#172536] to-[#0D151F]', version: '9.8.0' },
    { id: 'flowbytes', name: 'FlowBytes', category: 'AI & Dev', glyph: '⇅', monoTone: 'from-[#24242B] to-[#101014]', studioTone: 'from-[#152628] to-[#0B1416]', version: '1.1.0' },
    { id: 'forus', name: 'For Us', category: 'Productivity', glyph: 'Us', monoTone: 'from-[#2E2E36] to-[#151519]', studioTone: 'from-[#2E1E26] to-[#181014]', recent: true, version: '2.4.0' },
    { id: 'glovo', name: 'Glovo', category: 'Productivity', glyph: 'GLV', monoTone: 'from-[#E4E4E7] to-[#A1A1AA]', lightTile: true, studioTone: 'from-[#E5E0D5] to-[#C7C0B3]', version: '15.2' },
    { id: 'gmail', name: 'Gmail', category: 'Productivity', glyph: 'M', monoTone: 'from-[#2C2C33] to-[#141418]', studioTone: 'from-[#2E1E20] to-[#170F11]', version: '2026.05' },
    { id: 'google', name: 'Google', category: 'System', glyph: 'G', monoTone: 'from-[#F4F4F5] to-[#D4D4D8]', lightTile: true, studioTone: 'from-[#E4E4E7] to-[#C8C8D0]', version: '15.20' },
    { id: 'googleearth', name: 'Google Earth', category: 'Productivity', glyph: 'GEO', monoTone: 'from-[#2A2A32] to-[#131317]', studioTone: 'from-[#192836] to-[#0D161E]', version: '10.55' },
    { id: 'googleone', name: 'Google One', category: 'System', glyph: '1', monoTone: 'from-[#F4F4F5] to-[#D4D4D8]', lightTile: true, studioTone: 'from-[#E4E4E7] to-[#C8C8D0]', version: '1.210' },
    { id: 'googletv', name: 'Google TV', category: 'Media', glyph: 'TV', monoTone: 'from-[#25252C] to-[#111115]', studioTone: 'from-[#1B222C] to-[#0E1218]', version: '4.39' },
    { id: 'gpay', name: 'GPay', category: 'Productivity', glyph: '₹', monoTone: 'from-[#292931] to-[#131317]', studioTone: 'from-[#1A2436] to-[#0E141E]', version: '212.1' },
    { id: 'indrive', name: 'inDrive', category: 'Productivity', glyph: 'iD', monoTone: 'from-[#E4E4E7] to-[#A1A1AA]', lightTile: true, studioTone: 'from-[#DCE4D5] to-[#BEC8B5]', version: '5.72' },
    { id: 'instagram', name: 'Instagram', category: 'Media', glyph: 'IG', monoTone: 'from-[#2D2D35] to-[#141418]', studioTone: 'from-[#2E1E29] to-[#170F15]', version: '335.0' },
    { id: 'wallpapers', name: 'Interactive wallpapers', category: 'Media', glyph: 'WLP', monoTone: 'from-[#282830] to-[#121216]', studioTone: 'from-[#202036] to-[#11111E]', version: '1.8' },
    { id: 'keepnotes', name: 'Keep notes', category: 'Productivity', glyph: 'KPN', monoTone: 'from-[#E4E4E7] to-[#A1A1AA]', lightTile: true, studioTone: 'from-[#E6E1D6] to-[#C9C2B2]', version: '5.24' },
    { id: 'lawnchair', name: 'Lawnchair', category: 'System', glyph: 'LWN', monoTone: 'from-[#27272F] to-[#121216]', studioTone: 'from-[#182928] to-[#0D1716]', version: '14.0' },
    { id: 'lawnicons', name: 'Lawnicons', category: 'System', glyph: '⊞', monoTone: 'from-[#292931] to-[#131317]', studioTone: 'from-[#182534] to-[#0D151E]', version: '2.11' },
    { id: 'lens', name: 'Lens', category: 'AI & Dev', glyph: '⌖', monoTone: 'from-[#2B2B33] to-[#141418]', studioTone: 'from-[#1E1E34] to-[#10101D]', version: '1.17' },
    { id: 'lightroom', name: 'Lightroom', category: 'Media', glyph: 'Lr', monoTone: 'from-[#222229] to-[#0E0E12]', studioTone: 'from-[#152230] to-[#0B121A]', version: '9.3.1' },
    { id: 'localsend', name: 'LocalSend', category: 'System', glyph: '◎', monoTone: 'from-[#282830] to-[#121216]', studioTone: 'from-[#162827] to-[#0C1615]', version: '1.15.4' },
    { id: 'looop', name: 'Looop', category: 'Media', glyph: '∞', monoTone: 'from-[#2D2D36] to-[#141418]', studioTone: 'from-[#221E36] to-[#12101E]', recent: true, version: '3.0.2' },
    { id: 'makemytrip', name: 'MakeMyTrip', category: 'Productivity', glyph: 'my', monoTone: 'from-[#2B2B33] to-[#131317]', studioTone: 'from-[#2E1C22] to-[#170E12]', version: '9.14' },
    { id: 'maps', name: 'Maps', category: 'Productivity', glyph: 'MAP', monoTone: 'from-[#2A2A32] to-[#131317]', studioTone: 'from-[#172923] to-[#0C1613]', version: '11.132' },
    { id: 'meet', name: 'Meet', category: 'Productivity', glyph: 'MET', monoTone: 'from-[#2C2C34] to-[#141418]', studioTone: 'from-[#2B251B] to-[#16130E]', version: '245.0' },
    { id: 'messages', name: 'Messages', category: 'Productivity', glyph: 'MSG', monoTone: 'from-[#2E2E37] to-[#151519]', studioTone: 'from-[#1A2336] to-[#0E131E]', version: '2026.2' },
    { id: 'metrolist', name: 'Metrolist', category: 'Media', glyph: '♪', monoTone: 'from-[#26262D] to-[#111115]', studioTone: 'from-[#2B1B1B] to-[#160E0E]', version: '1.9.0' },
    { id: 'minecraft', name: 'Minecraft', category: 'Media', glyph: 'MC', monoTone: 'from-[#292930] to-[#131317]', studioTone: 'from-[#22291B] to-[#12160E]', version: '1.21' },
    { id: 'motioncues', name: 'Motion cues', category: 'System', glyph: '⋮⋮⋮', monoTone: 'from-[#2A2A33] to-[#131317]', studioTone: 'from-[#231D34] to-[#120F1C]', version: '1.0.5' },
    { id: 'moto', name: 'Moto', category: 'System', glyph: 'M', monoTone: 'from-[#24242C] to-[#101014]', studioTone: 'from-[#172032] to-[#0C111B]', version: '15.0.2' },
    { id: 'myjio', name: 'MyJio', category: 'System', glyph: 'Jio', monoTone: 'from-[#2B2B33] to-[#131317]', studioTone: 'from-[#2E1C1C] to-[#170E0E]', version: '7.0.9' },
    { id: 'openspace', name: 'Open Space', category: 'AI & Dev', glyph: '🪐', monoTone: 'from-[#23232A] to-[#0D0D10]', studioTone: 'from-[#1F182B] to-[#0D0B12]', recent: true, version: '2.8.0' },
    { id: 'perplexity', name: 'Perplexity', category: 'AI & Dev', glyph: '✵', monoTone: 'from-[#212127] to-[#0B0B0E]', studioTone: 'from-[#152428] to-[#0B1214]', recent: true, version: '2.28.0' }
];

const INITIAL_NOTIFICATIONS = [
    {
        id: 'n1',
        app: 'Messages',
        glyph: 'MSG',
        title: 'Lalit mukesh',
        body: 'Shared 3 high-resolution RAW captures via Smart Connect Direct.',
        time: '2m ago',
        tag: 'DIRECT',
        unread: true
    },
    {
        id: 'n2',
        app: 'Open Space',
        glyph: '🪐',
        title: 'Spatial Canvas Synced',
        body: 'Workspace environment mirrored at 120Hz zero-latency link.',
        time: '9m ago',
        tag: 'CONTINUITY',
        unread: true
    },
    {
        id: 'n3',
        app: 'Perplexity',
        glyph: '✵',
        title: 'Deep Research Completed',
        body: 'Monochrome Apple Pro architecture synthesis ready for review on PC.',
        time: '24m ago',
        tag: 'AI ENGINE',
        unread: false
    },
    {
        id: 'n4',
        app: 'Echo Music',
        glyph: '((◉))',
        title: 'ALAC 24-bit/192kHz Audio Route',
        body: 'Streaming lossless output from motorola edge 50 neo to Mac Studio.',
        time: '1h ago',
        tag: 'AUDIO',
        unread: false
    },
    {
        id: 'n5',
        app: 'Calendar',
        glyph: '31',
        title: 'Design Architecture Review',
        body: 'Starts in 15 minutes · Cross Control presentation mode ready.',
        time: '2h ago',
        tag: 'SCHEDULE',
        unread: false
    }
];

// ============================================================================
// REUSABLE MONOCHROME APPLE PRO COMPONENTS
// ============================================================================

/**
 * Pure Monochrome Apple Pro Toggle Switch
 * Active: #FFFFFF pill with #050506 thumb | Inactive: #1F1F24 with #71717A thumb
 */
function MonochromeToggle({ checked, onChange, disabled = false, label }) {
    return (
        <button
            type="button"
            role="switch"
            aria-checked={checked}
            aria-label={label}
            disabled={disabled}
            onClick={() => !disabled && onChange(!checked)}
            className={`relative inline-flex h-[24px] w-[44px] shrink-0 items-center rounded-full p-[2px] transition-all duration-200 ease-out focus:outline-none focus-visible:ring-2 focus-visible:ring-white/60 ${disabled ? 'opacity-35 cursor-not-allowed' : 'cursor-pointer'
                } ${checked
                    ? 'bg-white shadow-[0_0_16px_rgba(255,255,255,0.25)]'
                    : 'bg-[#1F1F24] border border-white/[0.08]'
                }`}
        >
            <span
                className={`pointer-events-none inline-block h-[18px] w-[18px] transform rounded-full transition-all duration-200 ease-out ${checked
                        ? 'translate-x-[20px] bg-[#050506] shadow-sm'
                        : 'translate-x-[1px] bg-[#A1A1AA]'
                    }`}
            />
        </button>
    );
}

/**
 * Apple Pro Elevated Bento Surface (#0E0E11 with 1px white/7 border & top specular highlight)
 */
function BentoSurface({ children, className = '' }) {
    return (
        <div
            className={`rounded-[14px] bg-[#0E0E11] border border-white/[0.07] shadow-[0_14px_40px_rgba(0,0,0,0.65),inset_0_1px_0_rgba(255,255,255,0.08)] overflow-hidden ${className}`}
        >
            {children}
        </div>
    );
}

export default function App() {
    // Primary Navigation State ('phone' | 'apps' | 'display' | 'files' | 'notifications')
    const [activeTab, setActiveTab] = useState('phone');

    // Connection State (Defaults to false to honor the MVP screenshots, toggleable anytime!)
    const [isConnected, setIsConnected] = useState(false);
    const [isConnecting, setIsConnecting] = useState(false);

    // Device Identity State
    const [deviceName, setDeviceName] = useState('motorola edge 50 neo');
    const [isEditingName, setIsEditingName] = useState(false);
    const [tempDeviceName, setTempDeviceName] = useState('motorola edge 50 neo');
    const [showDeviceDropdown, setShowDeviceDropdown] = useState(false);
    const [showSpecsModal, setShowSpecsModal] = useState(false);

    // Command Palette (⌘K) State
    const [isCmdPaletteOpen, setIsCmdPaletteOpen] = useState(false);
    const [cmdQuery, setCmdQuery] = useState('');

    // Apps View State
    const [appSearch, setAppSearch] = useState('');
    const [appCategory, setAppCategory] = useState('All 96');
    const [iconFinishMode, setIconFinishMode] = useState('monochrome'); // 'monochrome' | 'studio'
    const [appViewLayout, setAppViewLayout] = useState('grid'); // 'grid' | 'table'
    const [isRefreshingApps, setIsRefreshingApps] = useState(false);
    const [activeMirroredApp, setActiveMirroredApp] = useState(null);
    const appSearchRef = useRef(null);

    // Display / Cross Control State
    const [crossControlMode, setCrossControlMode] = useState('mouse'); // 'mouse' | 'duplicate' | 'extend'
    const [phonePosition, setPhonePosition] = useState('right'); // 'left' | 'right' | 'top' | 'bottom'
    const [phoneOffset, setPhoneOffset] = useState(0);
    const [isDraggingPhone, setIsDraggingPhone] = useState(false);
    const [phoneAsDisplayOn, setPhoneAsDisplayOn] = useState(false);
    const arrangementCanvasRef = useRef(null);

    // Files View State (Exact paths & defaults from MVP Screenshot 4)
    const [isStorageMounted, setIsStorageMounted] = useState(false);
    const [mountLocation, setMountLocation] = useState('/home/lalitmukesh/Phone (default)');
    const [autoMount, setAutoMount] = useState(true);
    const [shareFolderEnabled, setShareFolderEnabled] = useState(true);
    const [sharedFolderPath, setSharedFolderPath] = useState('/home/lalitmukesh (default)');
    const [includeHiddenFiles, setIncludeHiddenFiles] = useState(false);
    const [allowDeleteFiles, setAllowDeleteFiles] = useState(true);
    const [editingPathTarget, setEditingPathTarget] = useState(null); // 'mount' | 'shared' | null
    const [tempPathValue, setTempPathValue] = useState('');

    // Notifications View State (Defaults to 'empty' matching Screenshot 5, toggleable to 'live')
    const [notificationViewMode, setNotificationViewMode] = useState('empty'); // 'empty' | 'live'
    const [notifications, setNotifications] = useState(INITIAL_NOTIFICATIONS);
    const [replyingToId, setReplyingToId] = useState(null);
    const [replyText, setReplyText] = useState('');

    // Monochrome Toast Feedback HUD
    const [toastMessage, setToastMessage] = useState(null);

    const triggerToast = (title, subtitle = '') => {
        setToastMessage({ title, subtitle });
    };

    useEffect(() => {
        if (!toastMessage) return;
        const timer = setTimeout(() => setToastMessage(null), 3200);
        return () => clearTimeout(timer);
    }, [toastMessage]);

    // Global Keyboard Shortcuts (⌘K Command Palette, ⌘1-⌘5 Sidebar Views, Esc)
    useEffect(() => {
        const handleKeyDown = (e) => {
            if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                e.preventDefault();
                setIsCmdPaletteOpen((prev) => !prev);
                setCmdQuery('');
            } else if ((e.metaKey || e.ctrlKey) && e.key === '1') {
                e.preventDefault();
                setActiveTab('phone');
            } else if ((e.metaKey || e.ctrlKey) && e.key === '2') {
                e.preventDefault();
                setActiveTab('apps');
            } else if ((e.metaKey || e.ctrlKey) && e.key === '3') {
                e.preventDefault();
                setActiveTab('display');
            } else if ((e.metaKey || e.ctrlKey) && e.key === '4') {
                e.preventDefault();
                setActiveTab('files');
            } else if ((e.metaKey || e.ctrlKey) && e.key === '5') {
                e.preventDefault();
                setActiveTab('notifications');
            } else if (e.key === 'Escape') {
                setIsCmdPaletteOpen(false);
                setActiveMirroredApp(null);
                setEditingPathTarget(null);
                setShowSpecsModal(false);
            }
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, []);

    // Connection Handshake Toggle
    const handleToggleConnect = () => {
        if (isConnected) {
            setIsConnected(false);
            setIsStorageMounted(false);
            setPhoneAsDisplayOn(false);
            triggerToast('Disconnected · ' + deviceName, 'Continuity & SFTP tunnel closed safely');
        } else {
            setIsConnecting(true);
            setTimeout(() => {
                setIsConnecting(false);
                setIsConnected(true);
                if (autoMount) setIsStorageMounted(true);
                triggerToast('Connected · ' + deviceName, '1.2ms Ultra-Low Latency Continuity Active');
            }, 550);
        }
    };

    // Spatial Arrangement Pointer Drag Logic
    const handleArrangementPointerMove = (e) => {
        if (!isDraggingPhone || !arrangementCanvasRef.current) return;
        const rect = arrangementCanvasRef.current.getBoundingClientRect();
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 2;
        const dx = e.clientX - centerX;
        const dy = e.clientY - centerY;

        if (Math.abs(dx) > Math.abs(dy)) {
            setPhonePosition(dx < 0 ? 'left' : 'right');
            const clampedOffset = Math.max(-38, Math.min(38, Math.round(dy * 0.45)));
            setPhoneOffset(clampedOffset);
        } else {
            setPhonePosition(dy < 0 ? 'top' : 'bottom');
            const clampedOffset = Math.max(-60, Math.min(60, Math.round(dx * 0.45)));
            setPhoneOffset(clampedOffset);
        }
    };

    useEffect(() => {
        if (!isDraggingPhone) return;
        const handleUp = () => setIsDraggingPhone(false);
        window.addEventListener('pointerup', handleUp);
        return () => window.removeEventListener('pointerup', handleUp);
    }, [isDraggingPhone]);

    // Filtered Apps
    const filteredApps = useMemo(() => {
        return ALL_APPS_DATA.filter((app) => {
            const matchesSearch = app.name.toLowerCase().includes(appSearch.trim().toLowerCase());
            const matchesCategory =
                appCategory === 'All 96' ||
                (appCategory === 'Recent' && app.recent) ||
                app.category === appCategory;
            return matchesSearch && matchesCategory;
        });
    }, [appSearch, appCategory]);

    // Dynamic caption for Display view
    const phonePositionCaption = useMemo(() => {
        const captions = {
            right: "The phone is right of this PC's screen.",
            left: "The phone is left of this PC's screen.",
            top: "The phone is above this PC's screen.",
            bottom: "The phone is below this PC's screen."
        };
        return captions[phonePosition];
    }, [phonePosition]);

    // Save inline device rename
    const saveDeviceName = () => {
        const cleaned = tempDeviceName.trim() || 'motorola edge 50 neo';
        setDeviceName(cleaned);
        setTempDeviceName(cleaned);
        setIsEditingName(false);
        triggerToast('Identity Updated', `Device renamed to "${cleaned}"`);
    };

    // Refresh apps animation
    const handleRefreshApps = () => {
        setIsRefreshingApps(true);
        setTimeout(() => {
            setIsRefreshingApps(false);
            triggerToast('App Index Synchronized', `96 packages verified on ${deviceName}`);
        }, 550);
    };

    // Path Modal Helpers
    const openPathModal = (target) => {
        setEditingPathTarget(target);
        setTempPathValue(
            target === 'mount'
                ? mountLocation.replace(' (default)', '')
                : sharedFolderPath.replace(' (default)', '')
        );
    };

    const savePathModal = (e) => {
        e.preventDefault();
        if (!tempPathValue.trim()) return;
        if (editingPathTarget === 'mount') {
            setMountLocation(tempPathValue.trim());
            triggerToast('Mount Path Updated', tempPathValue.trim());
        } else {
            setSharedFolderPath(tempPathValue.trim());
            triggerToast('Shared Directory Updated', tempPathValue.trim());
        }
        setEditingPathTarget(null);
    };

    // Sidebar Navigation Structure
    const SIDEBAR_GROUPS = [
        {
            title: 'WORKSPACE',
            items: [
                {
                    id: 'phone',
                    label: 'Phone',
                    subtitle: 'Overview & Telemetry',
                    shortcut: '⌘1',
                    icon: Smartphone
                },
                {
                    id: 'apps',
                    label: 'Apps',
                    subtitle: 'Launchpad & Mirroring',
                    shortcut: '⌘2',
                    badge: '96',
                    icon: LayoutGrid
                }
            ]
        },
        {
            title: 'CONTINUITY',
            items: [
                {
                    id: 'display',
                    label: 'Display',
                    subtitle: 'Cross Control Studio',
                    shortcut: '⌘3',
                    icon: Monitor
                },
                {
                    id: 'files',
                    label: 'Files',
                    subtitle: 'Storage & SFTP Share',
                    shortcut: '⌘4',
                    icon: Folder
                },
                {
                    id: 'notifications',
                    label: 'Notifications',
                    subtitle: 'Alert Relay Center',
                    shortcut: '⌘5',
                    badge:
                        notificationViewMode === 'live' && notifications.filter((n) => n.unread).length > 0
                            ? String(notifications.filter((n) => n.unread).length)
                            : null,
                    icon: Bell
                }
            ]
        }
    ];

    const activeTabMeta = useMemo(() => {
        for (const group of SIDEBAR_GROUPS) {
            for (const item of group.items) {
                if (item.id === activeTab) return item;
            }
        }
        return SIDEBAR_GROUPS[0].items[0];
    }, [activeTab, notificationViewMode, notifications]);

    // Command Palette filtered actions
    const commandActions = useMemo(() => {
        const baseActions = [
            {
                id: 'cmd-connect',
                title: isConnected ? `Disconnect ${deviceName}` : `Connect ${deviceName}`,
                category: 'Continuity Link',
                shortcut: 'Enter',
                action: () => handleToggleConnect()
            },
            {
                id: 'cmd-phone',
                title: 'Go to Phone — Hardware Overview & Telemetry',
                category: 'Navigation',
                shortcut: '⌘1',
                action: () => setActiveTab('phone')
            },
            {
                id: 'cmd-apps',
                title: 'Go to Apps — Browse & Stream 96 Apps',
                category: 'Navigation',
                shortcut: '⌘2',
                action: () => setActiveTab('apps')
            },
            {
                id: 'cmd-display',
                title: 'Go to Display — Cross Control & Spatial Arrangement',
                category: 'Navigation',
                shortcut: '⌘3',
                action: () => setActiveTab('display')
            },
            {
                id: 'cmd-files',
                title: 'Go to Files — Phone Storage & Shared Folder',
                category: 'Navigation',
                shortcut: '⌘4',
                action: () => setActiveTab('files')
            },
            {
                id: 'cmd-notifications',
                title: 'Go to Notifications — Continuity Relay',
                category: 'Navigation',
                shortcut: '⌘5',
                action: () => setActiveTab('notifications')
            },
            {
                id: 'cmd-mount',
                title: isStorageMounted
                    ? 'Unmount /home/lalitmukesh/Phone'
                    : 'Mount /home/lalitmukesh/Phone (Read-Only)',
                category: 'Storage',
                shortcut: 'SFTP',
                action: () => {
                    if (!isConnected) setIsConnected(true);
                    setIsStorageMounted((prev) => !prev);
                    setActiveTab('files');
                    triggerToast('Storage Mount Toggled', mountLocation);
                }
            }
        ];

        const appActions = ALL_APPS_DATA.slice(0, 25).map((app) => ({
            id: `app-${app.id}`,
            title: `Launch ${app.name}`,
            category: `App Stream · ${app.category}`,
            shortcut: 'Stream',
            action: () => {
                setActiveTab('apps');
                setActiveMirroredApp(app);
            }
        }));

        const all = [...baseActions, ...appActions];
        if (!cmdQuery.trim()) return all.slice(0, 10);
        return all.filter(
            (item) =>
                item.title.toLowerCase().includes(cmdQuery.toLowerCase()) ||
                item.category.toLowerCase().includes(cmdQuery.toLowerCase())
        );
    }, [cmdQuery, isConnected, isStorageMounted, deviceName, mountLocation]);

    return (
        <div
            className="w-screen h-screen bg-[#050506] text-[#F4F4F6] select-none flex flex-col md:flex-row overflow-hidden"
            style={{
                fontFamily:
                    '-apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text", "Inter", system-ui, sans-serif'
            }}
        >
            {/* ===================================================================
          LEFT SIDEBAR (272px Fixed Width on Desktop · Pure Full-Bleed UI)
      =================================================================== */}
            <aside className="w-full md:w-[272px] shrink-0 bg-[#0A0A0C] border-b md:border-b-0 md:border-r border-white/[0.07] flex flex-col justify-between z-30">
                {/* Top Section: System Brand Header + Device Switcher + Navigation */}
                <div className="p-4 space-y-5">
                    {/* 1. Pure SaaS Workspace Header + Quick ⌘K Trigger */}
                    <div className="flex items-center justify-between px-1 pt-0.5">
                        <div className="flex items-center gap-2">
                            <div className="w-5 h-5 rounded-[6px] bg-white text-[#050506] flex items-center justify-center">
                                <Layers className="w-3 h-3 stroke-[2.5]" />
                            </div>
                            <span className="text-[11px] font-mono font-semibold uppercase tracking-[0.14em] text-[#E4E4E7]">
                                Smart Connect
                            </span>
                        </div>

                        <button
                            onClick={() => setIsCmdPaletteOpen(true)}
                            title="Command Palette (⌘K)"
                            className="flex items-center gap-1 px-2 py-1 rounded-[7px] bg-[#141418] hover:bg-[#1E1E24] border border-white/[0.07] text-[11px] font-mono text-[#A1A1AA] hover:text-white transition-colors"
                        >
                            <Command className="w-3 h-3" />
                            <span>K</span>
                        </button>
                    </div>

                    {/* 2. Interactive Device Switcher Card */}
                    <div className="relative">
                        <button
                            onClick={() => setShowDeviceDropdown(!showDeviceDropdown)}
                            className="w-full p-3 rounded-[12px] bg-[#121216] hover:bg-[#18181E] border border-white/[0.08] shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] transition-all duration-150 flex items-center justify-between gap-2.5 text-left group"
                        >
                            <div className="flex items-center gap-3 min-w-0">
                                <div className="relative w-9 h-9 rounded-[9px] bg-[#1C1C22] border border-white/[0.1] flex items-center justify-center shrink-0">
                                    <Smartphone className="w-4 h-4 text-white/90" />
                                    <span
                                        className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-[#121216] ${isConnected
                                                ? 'bg-white shadow-[0_0_8px_#FFFFFF]'
                                                : isConnecting
                                                    ? 'bg-[#A1A1AA] animate-pulse'
                                                    : 'bg-[#52525B]'
                                            }`}
                                    />
                                </div>

                                <div className="min-w-0">
                                    <div className="text-[13px] font-semibold tracking-[-0.01em] text-white truncate">
                                        {deviceName}
                                    </div>
                                    <div className="flex items-center gap-1.5 text-[11px] font-mono text-[#A1A1AA] mt-0.5">
                                        <span>{isConnected ? 'Connected · 1.2ms' : 'Disconnected · 2d'}</span>
                                        <span>·</span>
                                        <span className="text-white/80">60%</span>
                                    </div>
                                </div>
                            </div>

                            <ChevronDown className="w-3.5 h-3.5 text-[#71717A] group-hover:text-white transition-colors shrink-0" />
                        </button>

                        {/* Device Switcher Popover */}
                        {showDeviceDropdown && (
                            <>
                                <div
                                    className="fixed inset-0 z-40"
                                    onClick={() => setShowDeviceDropdown(false)}
                                />
                                <div className="absolute left-0 right-0 mt-1.5 rounded-[12px] bg-[#141418] border border-white/[0.12] shadow-[0_20px_50px_rgba(0,0,0,0.85)] p-1.5 z-50 text-[12.5px]">
                                    <div className="px-2.5 py-1 text-[10px] font-mono uppercase tracking-[0.12em] text-[#71717A]">
                                        Paired Endpoint
                                    </div>
                                    <button
                                        onClick={() => setShowDeviceDropdown(false)}
                                        className="w-full flex items-center justify-between px-2.5 py-2 rounded-[8px] bg-white text-[#050506] font-medium"
                                    >
                                        <div className="flex items-center gap-2 min-w-0">
                                            <Smartphone className="w-3.5 h-3.5 shrink-0" />
                                            <span className="truncate">{deviceName}</span>
                                        </div>
                                        <Check className="w-3.5 h-3.5 shrink-0" />
                                    </button>

                                    <div className="my-1 border-t border-white/[0.07]" />

                                    <button
                                        onClick={() => {
                                            setShowDeviceDropdown(false);
                                            setActiveTab('phone');
                                            setTempDeviceName(deviceName);
                                            setIsEditingName(true);
                                        }}
                                        className="w-full flex items-center justify-between px-2.5 py-1.5 rounded-[7px] hover:bg-white/[0.06] text-[#E4E4E7]"
                                    >
                                        <span>Rename Device…</span>
                                        <Pencil className="w-3 h-3 text-[#71717A]" />
                                    </button>
                                    <button
                                        onClick={() => {
                                            setShowDeviceDropdown(false);
                                            setShowSpecsModal(true);
                                        }}
                                        className="w-full flex items-center justify-between px-2.5 py-1.5 rounded-[7px] hover:bg-white/[0.06] text-[#E4E4E7]"
                                    >
                                        <span>System Telemetry…</span>
                                        <Info className="w-3 h-3 text-[#71717A]" />
                                    </button>
                                </div>
                            </>
                        )}
                    </div>

                    {/* 3. Sidebar Navigation Groups (WORKSPACE & CONTINUITY) */}
                    <nav aria-label="Sidebar Navigation" className="space-y-5">
                        {SIDEBAR_GROUPS.map((group) => (
                            <div key={group.title} className="space-y-1">
                                <div className="px-2.5 pb-1 text-[10px] font-mono font-semibold uppercase tracking-[0.14em] text-[#71717A]">
                                    {group.title}
                                </div>
                                <div className="flex md:flex-col gap-1 overflow-x-auto md:overflow-visible pb-1 md:pb-0">
                                    {group.items.map((item) => {
                                        const Icon = item.icon;
                                        const isActive = activeTab === item.id;
                                        return (
                                            <button
                                                key={item.id}
                                                onClick={() => setActiveTab(item.id)}
                                                className={`group relative w-full flex items-center justify-between gap-2.5 px-3 py-2.5 rounded-[10px] text-left transition-all duration-150 shrink-0 md:shrink ${isActive
                                                        ? 'bg-white text-[#050506] shadow-[0_4px_20px_rgba(255,255,255,0.14)] font-semibold'
                                                        : 'text-[#A1A1AA] hover:text-white hover:bg-[#141418]'
                                                    }`}
                                            >
                                                <div className="flex items-center gap-3 min-w-0">
                                                    <Icon
                                                        className={`w-4 h-4 shrink-0 transition-colors ${isActive
                                                                ? 'text-[#050506]'
                                                                : 'text-[#71717A] group-hover:text-white'
                                                            }`}
                                                    />
                                                    <div className="min-w-0">
                                                        <div className="text-[13px] leading-tight truncate">
                                                            {item.label}
                                                        </div>
                                                        <div
                                                            className={`hidden md:block text-[11px] truncate mt-0.5 ${isActive ? 'text-[#050506]/70 font-medium' : 'text-[#71717A]'
                                                                }`}
                                                        >
                                                            {item.subtitle}
                                                        </div>
                                                    </div>
                                                </div>

                                                <div className="hidden md:flex items-center gap-1.5 shrink-0">
                                                    {item.badge && (
                                                        <span
                                                            className={`px-1.5 py-0.5 rounded-[5px] text-[10px] font-mono font-semibold ${isActive
                                                                    ? 'bg-[#050506] text-white'
                                                                    : 'bg-[#1F1F24] text-[#A1A1AA] border border-white/[0.06]'
                                                                }`}
                                                        >
                                                            {item.badge}
                                                        </span>
                                                    )}
                                                    <span
                                                        className={`text-[10px] font-mono ${isActive ? 'text-[#050506]/60' : 'text-[#52525B]'
                                                            }`}
                                                    >
                                                        {item.shortcut}
                                                    </span>
                                                </div>
                                            </button>
                                        );
                                    })}
                                </div>
                            </div>
                        ))}
                    </nav>
                </div>

                {/* 4. Sidebar Footer Widget (Live Telemetry Summary + Master Connect Action) */}
                <div className="hidden md:block p-4 border-t border-white/[0.07] bg-[#08080A] space-y-3">
                    <div className="flex items-center justify-between text-[11px] font-mono text-[#A1A1AA] px-1">
                        <span className="flex items-center gap-1.5">
                            <Battery className="w-3.5 h-3.5 text-white" /> 60%
                        </span>
                        <span>·</span>
                        <span className="flex items-center gap-1">
                            <Wifi className="w-3 h-3 text-[#A1A1AA]" /> Weak
                        </span>
                        <span>·</span>
                        <span className="flex items-center gap-1">
                            <Bluetooth className="w-3 h-3 text-white" /> On
                        </span>
                    </div>

                    <button
                        onClick={handleToggleConnect}
                        disabled={isConnecting}
                        className={`w-full py-2.5 px-4 rounded-[10px] text-[12.5px] font-semibold tracking-[-0.01em] transition-all duration-150 flex items-center justify-center gap-2 ${isConnected
                                ? 'bg-[#16161A] hover:bg-[#1F1F25] text-white border border-white/[0.12]'
                                : 'bg-white hover:bg-[#E4E4E7] text-[#050506] shadow-[0_0_25px_rgba(255,255,255,0.18)]'
                            }`}
                    >
                        {isConnecting ? (
                            <>
                                <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                                <span>Handshaking…</span>
                            </>
                        ) : isConnected ? (
                            <>
                                <span className="w-2 h-2 rounded-full bg-white animate-pulse" />
                                <span>Disconnect Device</span>
                            </>
                        ) : (
                            <>
                                <span>Connect Device</span>
                                <ArrowUpRight className="w-3.5 h-3.5" />
                            </>
                        )}
                    </button>
                </div>
            </aside>

            {/* ===================================================================
          MAIN WORKSPACE AREA (Full-Height Flex-1 + Sticky Header + Bento Canvas)
      =================================================================== */}
            <div className="flex-1 flex flex-col min-w-0 h-full bg-[#050506] overflow-hidden">
                {/* TOP GLASS HEADER BAR (56px Height) */}
                <header className="h-[56px] shrink-0 px-5 sm:px-8 bg-[#050506]/85 backdrop-blur-2xl border-b border-white/[0.07] flex items-center justify-between gap-4 z-20">
                    {/* Left: Breadcrumb */}
                    <div className="flex items-center gap-2 text-[13px] min-w-0">
                        <span className="text-[#71717A] truncate hidden sm:inline">{deviceName}</span>
                        <span className="text-[#3F3F46] hidden sm:inline">/</span>
                        <span className="font-semibold text-white tracking-[-0.01em]">
                            {activeTabMeta.label}
                        </span>
                        <span className="hidden lg:inline-block ml-2 px-2 py-0.5 rounded-[6px] bg-[#121216] border border-white/[0.06] text-[11px] font-mono text-[#A1A1AA]">
                            {isConnected
                                ? 'Connected · TLS 1.3 Active'
                                : 'Disconnected · last connected 2 days ago'}
                        </span>
                    </div>

                    {/* Right: Command Palette Trigger + Master Connect Button */}
                    <div className="flex items-center gap-2.5 shrink-0">
                        <button
                            onClick={() => setIsCmdPaletteOpen(true)}
                            className="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-[8px] bg-[#0E0E11] hover:bg-[#16161A] border border-white/[0.08] text-[12px] text-[#A1A1AA] hover:text-white transition-colors"
                        >
                            <Search className="w-3.5 h-3.5" />
                            <span>Quick Command…</span>
                            <kbd className="ml-1 px-1.5 py-0.2 rounded bg-white/[0.06] font-mono text-[10.5px] text-white/70">
                                ⌘K
                            </kbd>
                        </button>

                        <button
                            onClick={() => setShowSpecsModal(true)}
                            title="System Specifications"
                            className="p-2 rounded-[8px] bg-[#0E0E11] hover:bg-[#16161A] border border-white/[0.08] text-[#A1A1AA] hover:text-white transition-colors"
                        >
                            <Info className="w-4 h-4" />
                        </button>

                        <button
                            onClick={handleToggleConnect}
                            disabled={isConnecting}
                            className={`px-4 py-1.5 rounded-[8px] text-[12.5px] font-semibold tracking-[-0.01em] transition-all flex items-center gap-2 ${isConnected
                                    ? 'bg-[#16161A] hover:bg-[#1F1F25] text-white border border-white/[0.12]'
                                    : 'bg-white hover:bg-[#E4E4E7] text-[#050506] shadow-[0_0_20px_rgba(255,255,255,0.18)]'
                                }`}
                        >
                            {isConnecting && <RefreshCw className="w-3.5 h-3.5 animate-spin" />}
                            <span>{isConnecting ? 'Connecting…' : isConnected ? 'Disconnect' : 'Connect'}</span>
                        </button>
                    </div>
                </header>

                {/* SCROLLABLE BENTO WORKSPACE CANVAS */}
                <main className="flex-1 overflow-y-auto px-5 sm:px-8 py-6 sm:py-8">
                    {/* ===============================================================
              VIEW A: PHONE (PURE UI DEVICE IDENTITY & TELEMETRY BENTO)
          =============================================================== */}
                    {activeTab === 'phone' && (
                        <div className="max-w-[1040px] mx-auto space-y-6">
                            {/* 1. PURE UI DEVICE IDENTITY & TELEMETRY CARD (Zero Hardware Frame Mockups) */}
                            <BentoSurface className="p-6 sm:p-8">
                                <div className="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-6">
                                    {/* Left: Pure UI Endpoint Identity & Status */}
                                    <div className="flex items-start gap-5 min-w-0 flex-1">
                                        <div className="w-14 h-14 rounded-[14px] bg-[#16161B] border border-white/[0.1] flex items-center justify-center shrink-0 shadow-[inset_0_1px_0_rgba(255,255,255,0.1)]">
                                            <Smartphone className="w-6 h-6 text-white" />
                                        </div>

                                        <div className="space-y-2.5 min-w-0 flex-1">
                                            <div className="flex items-center gap-2 flex-wrap">
                                                <span className="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-[#16161A] border border-white/[0.08] text-[10.5px] font-mono uppercase tracking-[0.12em] text-[#A1A1AA]">
                                                    <span
                                                        className={`w-1.5 h-1.5 rounded-full ${isConnected ? 'bg-white shadow-[0_0_8px_#FFFFFF]' : 'bg-[#71717A]'
                                                            }`}
                                                    />
                                                    <span>
                                                        {isConnected ? 'Continuity Session Online' : 'Endpoint Standby'}
                                                    </span>
                                                </span>

                                                <span className="px-2 py-0.5 rounded-full bg-[#121216] border border-white/[0.06] text-[10.5px] font-mono text-[#71717A]">
                                                    TLS 1.3 · SFTP · 120Hz
                                                </span>
                                            </div>

                                            {/* Device Name + Inline Edit */}
                                            <div className="flex items-center gap-3 flex-wrap">
                                                {isEditingName ? (
                                                    <div className="flex items-center gap-2">
                                                        <input
                                                            type="text"
                                                            value={tempDeviceName}
                                                            onChange={(e) => setTempDeviceName(e.target.value)}
                                                            onKeyDown={(e) => e.key === 'Enter' && saveDeviceName()}
                                                            autoFocus
                                                            className="px-3 py-1 rounded-[9px] bg-[#050506] border border-white text-[22px] font-semibold tracking-[-0.025em] text-white focus:outline-none"
                                                        />
                                                        <button
                                                            onClick={saveDeviceName}
                                                            className="p-2 rounded-[9px] bg-white text-[#050506] hover:bg-[#E4E4E7]"
                                                            title="Save Name"
                                                        >
                                                            <Check className="w-4 h-4" />
                                                        </button>
                                                    </div>
                                                ) : (
                                                    <>
                                                        <h1 className="text-[26px] sm:text-[30px] font-semibold tracking-[-0.03em] text-white leading-tight">
                                                            {deviceName}
                                                        </h1>
                                                        <button
                                                            onClick={() => {
                                                                setTempDeviceName(deviceName);
                                                                setIsEditingName(true);
                                                            }}
                                                            title="Rename device"
                                                            className="p-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#222228] border border-white/[0.07] text-[#A1A1AA] hover:text-white transition-colors"
                                                        >
                                                            <Pencil className="w-3.5 h-3.5" />
                                                        </button>
                                                    </>
                                                )}
                                            </div>

                                            {/* Exact Status Subtitle from MVP */}
                                            <p className="text-[13.5px] text-[#A1A1AA] font-normal">
                                                {isConnecting
                                                    ? 'Negotiating encrypted peer-to-peer display & storage link…'
                                                    : isConnected
                                                        ? 'Connected · Zero-latency Cross Control & SFTP Read-Only Mount active'
                                                        : 'Disconnected · last connected 2 days ago'}
                                            </p>
                                        </div>
                                    </div>

                                    {/* Right Action Controls */}
                                    <div className="flex items-center gap-2.5 w-full lg:w-auto shrink-0">
                                        <button
                                            onClick={() => setShowSpecsModal(true)}
                                            className="px-4 py-2.5 rounded-[10px] bg-[#16161A] hover:bg-[#1E1E24] border border-white/[0.08] text-[12.5px] font-medium text-[#A1A1AA] hover:text-white transition-colors flex items-center justify-center gap-2"
                                        >
                                            <Info className="w-3.5 h-3.5" />
                                            <span>Telemetry</span>
                                        </button>

                                        <button
                                            onClick={handleToggleConnect}
                                            disabled={isConnecting}
                                            className={`flex-1 lg:flex-initial px-6 py-2.5 rounded-[10px] text-[13px] font-semibold tracking-[-0.01em] transition-all flex items-center justify-center gap-2 ${isConnected
                                                    ? 'bg-[#16161A] hover:bg-[#1E1E24] text-white border border-white/[0.14]'
                                                    : 'bg-white hover:bg-[#E4E4E7] text-[#050506] shadow-[0_0_25px_rgba(255,255,255,0.2)]'
                                                }`}
                                        >
                                            {isConnecting && <RefreshCw className="w-4 h-4 animate-spin" />}
                                            <span>
                                                {isConnecting ? 'Connecting…' : isConnected ? 'Disconnect' : 'Connect'}
                                            </span>
                                        </button>
                                    </div>
                                </div>

                                {/* 4-COLUMN MONOCHROME TELEMETRY GRID (60% Battery, Weak Wi-Fi, Fair Cellular, Bluetooth On) */}
                                <div className="mt-7 pt-6 border-t border-white/[0.07] grid grid-cols-2 lg:grid-cols-4 gap-3.5">
                                    {/* Telemetry 1: Battery 60% */}
                                    <div className="p-4 rounded-[12px] bg-[#131317] border border-white/[0.06] flex flex-col justify-between gap-3">
                                        <div className="flex items-center justify-between text-[#A1A1AA]">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.1em]">
                                                BATTERY
                                            </span>
                                            <Battery className="w-4 h-4 text-white" />
                                        </div>
                                        <div>
                                            <div className="text-[22px] font-semibold tracking-[-0.02em] text-white font-mono">
                                                60%
                                            </div>
                                            <div className="mt-2 grid grid-cols-5 gap-1">
                                                <span className="h-1.5 rounded-sm bg-white" />
                                                <span className="h-1.5 rounded-sm bg-white" />
                                                <span className="h-1.5 rounded-sm bg-white" />
                                                <span className="h-1.5 rounded-sm bg-white/15" />
                                                <span className="h-1.5 rounded-sm bg-white/15" />
                                            </div>
                                        </div>
                                    </div>

                                    {/* Telemetry 2: Wi-Fi Weak */}
                                    <div className="p-4 rounded-[12px] bg-[#131317] border border-white/[0.06] flex flex-col justify-between gap-3">
                                        <div className="flex items-center justify-between text-[#A1A1AA]">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.1em]">
                                                WI-FI LINK
                                            </span>
                                            <Wifi className="w-4 h-4 text-[#A1A1AA]" />
                                        </div>
                                        <div>
                                            <div className="text-[22px] font-semibold tracking-[-0.02em] text-white">
                                                {isConnected ? 'Optimal' : 'Weak'}
                                            </div>
                                            <div className="text-[11.5px] font-mono text-[#71717A] mt-1">
                                                5 GHz · -74 dBm
                                            </div>
                                        </div>
                                    </div>

                                    {/* Telemetry 3: Cellular Fair */}
                                    <div className="p-4 rounded-[12px] bg-[#131317] border border-white/[0.06] flex flex-col justify-between gap-3">
                                        <div className="flex items-center justify-between text-[#A1A1AA]">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.1em]">
                                                CELLULAR
                                            </span>
                                            <Signal className="w-4 h-4 text-[#A1A1AA]" />
                                        </div>
                                        <div>
                                            <div className="text-[22px] font-semibold tracking-[-0.02em] text-white">
                                                Fair
                                            </div>
                                            <div className="mt-2 flex items-end gap-1 h-2.5">
                                                <span className="w-2 h-1.5 bg-white rounded-[1px]" />
                                                <span className="w-2 h-2 bg-white rounded-[1px]" />
                                                <span className="w-2 h-2.5 bg-white/20 rounded-[1px]" />
                                                <span className="w-2 h-3 bg-white/20 rounded-[1px]" />
                                                <span className="ml-1.5 text-[11px] font-mono text-[#71717A] leading-none">
                                                    5G NR
                                                </span>
                                            </div>
                                        </div>
                                    </div>

                                    {/* Telemetry 4: Bluetooth On */}
                                    <div className="p-4 rounded-[12px] bg-[#131317] border border-white/[0.06] flex flex-col justify-between gap-3">
                                        <div className="flex items-center justify-between text-[#A1A1AA]">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.1em]">
                                                BLUETOOTH
                                            </span>
                                            <Bluetooth className="w-4 h-4 text-white" />
                                        </div>
                                        <div>
                                            <div className="text-[22px] font-semibold tracking-[-0.02em] text-white">
                                                On
                                            </div>
                                            <div className="text-[11.5px] font-mono text-[#71717A] mt-1">
                                                BLE 5.3 · Proximity Ready
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </BentoSurface>

                            {/* 2. CROSS CONTROL QUICK-LAUNCH BENTO CARD */}
                            <section className="space-y-2.5">
                                <div className="px-1 flex items-center justify-between">
                                    <h2 className="text-[12px] font-mono font-semibold uppercase tracking-[0.14em] text-[#71717A]">
                                        Cross Control
                                    </h2>
                                    <span className="text-[11.5px] font-mono text-[#71717A]">
                                        Universal Control & Spatial Display
                                    </span>
                                </div>

                                <BentoSurface>
                                    <div
                                        onClick={() => setActiveTab('display')}
                                        className="p-5 sm:px-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4 hover:bg-[#141418] transition-colors cursor-pointer group"
                                    >
                                        <div className="flex items-center gap-4 min-w-0">
                                            <div className="w-11 h-11 rounded-[11px] bg-white text-[#050506] flex items-center justify-center shrink-0 shadow-[0_0_20px_rgba(255,255,255,0.2)]">
                                                <MousePointer className="w-5 h-5" />
                                            </div>
                                            <div className="min-w-0">
                                                <div className="flex items-center gap-2.5 flex-wrap">
                                                    <span className="text-[16px] font-semibold tracking-[-0.015em] text-white">
                                                        Cross Control
                                                    </span>
                                                    <span className="px-2 py-0.5 rounded-[6px] bg-[#16161A] border border-white/[0.08] text-[11px] font-mono text-[#A1A1AA]">
                                                        Edge: {phonePosition.toUpperCase()}
                                                    </span>
                                                </div>
                                                <p className="text-[13.5px] text-[#A1A1AA] mt-0.5">
                                                    Shared mouse and keyboard, duplicate display or extend display
                                                </p>
                                            </div>
                                        </div>

                                        <div className="flex items-center gap-2 shrink-0 self-start sm:self-center">
                                            {['Pointer', 'Mirror', 'Extend'].map((pill, idx) => {
                                                const modeKey =
                                                    idx === 0 ? 'mouse' : idx === 1 ? 'duplicate' : 'extend';
                                                const active = crossControlMode === modeKey;
                                                return (
                                                    <span
                                                        key={pill}
                                                        className={`px-2.5 py-1 rounded-[7px] text-[11.5px] font-mono transition-colors ${active
                                                                ? 'bg-white text-[#050506] font-semibold'
                                                                : 'bg-[#16161A] text-[#71717A] border border-white/[0.06]'
                                                            }`}
                                                    >
                                                        {pill}
                                                    </span>
                                                );
                                            })}
                                            <ChevronRight className="w-4 h-4 text-[#71717A] group-hover:text-white group-hover:translate-x-0.5 transition-all ml-1" />
                                        </div>
                                    </div>
                                </BentoSurface>
                            </section>

                            {/* 3. QUICK SYSTEM DIAGNOSTICS & RECENT ACTIVITY STRIP */}
                            <section className="space-y-2.5">
                                <div className="px-1">
                                    <h2 className="text-[12px] font-mono font-semibold uppercase tracking-[0.14em] text-[#71717A]">
                                        System Continuity Diagnostics
                                    </h2>
                                </div>

                                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    {/* Strip 1: Last Streamed App */}
                                    <BentoSurface className="p-5 flex flex-col justify-between gap-4 hover:border-white/20 transition-all">
                                        <div className="flex items-center justify-between">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.12em] text-[#71717A]">
                                                RECENT APP STREAM
                                            </span>
                                            <LayoutGrid className="w-4 h-4 text-[#A1A1AA]" />
                                        </div>
                                        <div className="flex items-center justify-between gap-3">
                                            <div className="flex items-center gap-3 min-w-0">
                                                <div className="w-10 h-10 rounded-[22%] bg-[#1C1C22] border border-white/[0.12] flex items-center justify-center font-mono text-[12px] font-bold text-white shrink-0">
                                                    ((◉))
                                                </div>
                                                <div className="min-w-0">
                                                    <div className="text-[14.5px] font-semibold text-white truncate">
                                                        Echo Music
                                                    </div>
                                                    <div className="text-[12px] text-[#71717A] truncate">
                                                        96 Apps Indexed
                                                    </div>
                                                </div>
                                            </div>
                                            <button
                                                onClick={() => setActiveTab('apps')}
                                                className="px-3 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-white hover:text-[#050506] text-[12px] font-medium text-white border border-white/[0.08] transition-colors shrink-0"
                                            >
                                                Open
                                            </button>
                                        </div>
                                    </BentoSurface>

                                    {/* Strip 2: Last Mounted Storage Path */}
                                    <BentoSurface className="p-5 flex flex-col justify-between gap-4 hover:border-white/20 transition-all">
                                        <div className="flex items-center justify-between">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.12em] text-[#71717A]">
                                                MOUNT LOCATION
                                            </span>
                                            <HardDrive className="w-4 h-4 text-[#A1A1AA]" />
                                        </div>
                                        <div className="flex items-center justify-between gap-3">
                                            <div className="min-w-0">
                                                <div className="text-[13px] font-mono text-white truncate">
                                                    /home/lalitmukesh/Phone
                                                </div>
                                                <div className="text-[12px] text-[#71717A] mt-0.5">
                                                    {isStorageMounted ? 'Mounted Read-Only' : 'Auto-mount ready'}
                                                </div>
                                            </div>
                                            <button
                                                onClick={() => setActiveTab('files')}
                                                className="px-3 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-white hover:text-[#050506] text-[12px] font-medium text-white border border-white/[0.08] transition-colors shrink-0"
                                            >
                                                Files
                                            </button>
                                        </div>
                                    </BentoSurface>

                                    {/* Strip 3: Host Shared Folder */}
                                    <BentoSurface className="p-5 flex flex-col justify-between gap-4 hover:border-white/20 transition-all">
                                        <div className="flex items-center justify-between">
                                            <span className="text-[11px] font-mono uppercase tracking-[0.12em] text-[#71717A]">
                                                SHARED DIRECTORY
                                            </span>
                                            <FolderOpen className="w-4 h-4 text-[#A1A1AA]" />
                                        </div>
                                        <div className="flex items-center justify-between gap-3">
                                            <div className="min-w-0">
                                                <div className="text-[13px] font-mono text-white truncate">
                                                    /home/lalitmukesh
                                                </div>
                                                <div className="text-[12px] text-[#71717A] mt-0.5">
                                                    {shareFolderEnabled ? 'Visible in Phone Files app' : 'Sharing paused'}
                                                </div>
                                            </div>
                                            <button
                                                onClick={() => setActiveTab('files')}
                                                className="px-3 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-white hover:text-[#050506] text-[12px] font-medium text-white border border-white/[0.08] transition-colors shrink-0"
                                            >
                                                Configure
                                            </button>
                                        </div>
                                    </BentoSurface>
                                </div>
                            </section>
                        </div>
                    )}

                    {/* ===============================================================
              VIEW B: APPS (MONOCHROME LAUNCHPAD & STREAM DIALOG)
          =============================================================== */}
                    {activeTab === 'apps' && (
                        <div className="max-w-[1060px] mx-auto space-y-6">
                            {/* 1. TOP CONTROL BAR */}
                            <BentoSurface className="p-4 flex flex-col lg:flex-row items-stretch lg:items-center justify-between gap-3.5">
                                {/* Instant Search Input */}
                                <div className="relative flex-1 max-w-full lg:max-w-[360px]">
                                    <Search className="w-4 h-4 text-[#71717A] absolute left-3.5 top-1/2 -translate-y-1/2 pointer-events-none" />
                                    <input
                                        ref={appSearchRef}
                                        type="text"
                                        value={appSearch}
                                        onChange={(e) => setAppSearch(e.target.value)}
                                        placeholder="Search 96 apps..."
                                        className="w-full pl-9 pr-9 py-2 rounded-[9px] bg-[#08080A] border border-white/[0.09] text-[13px] text-white placeholder-[#71717A] focus:outline-none focus:border-white transition-colors"
                                    />
                                    {appSearch && (
                                        <button
                                            onClick={() => setAppSearch('')}
                                            className="absolute right-2.5 top-1/2 -translate-y-1/2 p-1 text-[#71717A] hover:text-white"
                                        >
                                            <X className="w-3.5 h-3.5" />
                                        </button>
                                    )}
                                </div>

                                {/* Category Filter Pills */}
                                <div className="flex items-center gap-1.5 overflow-x-auto pb-1 lg:pb-0">
                                    {['All 96', 'Recent', 'AI & Dev', 'Productivity', 'Media', 'System'].map(
                                        (cat) => (
                                            <button
                                                key={cat}
                                                onClick={() => setAppCategory(cat)}
                                                className={`px-3 py-1.5 rounded-[8px] text-[12px] font-medium transition-all whitespace-nowrap ${appCategory === cat
                                                        ? 'bg-white text-[#050506] font-semibold shadow-sm'
                                                        : 'bg-[#16161A] text-[#A1A1AA] hover:text-white border border-white/[0.06]'
                                                    }`}
                                            >
                                                {cat}
                                            </button>
                                        )
                                    )}
                                </div>

                                {/* Right Controls: Monochrome Finish Mode Toggle + Grid/Table Switcher + Refresh */}
                                <div className="flex items-center gap-2 shrink-0">
                                    <button
                                        onClick={() =>
                                            setIconFinishMode(
                                                iconFinishMode === 'monochrome' ? 'studio' : 'monochrome'
                                            )
                                        }
                                        title="Switch between Pure Monochrome Glyph Mode and Subtle Desaturated Studio Mode"
                                        className="px-2.5 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#1E1E24] border border-white/[0.08] text-[11.5px] font-mono text-[#E4E4E7] flex items-center gap-1.5 transition-colors"
                                    >
                                        <Sparkles className="w-3.5 h-3.5 text-white" />
                                        <span>
                                            {iconFinishMode === 'monochrome' ? 'Pure Mono' : 'Studio Tint'}
                                        </span>
                                    </button>

                                    <div className="flex items-center p-0.5 rounded-[8px] bg-[#08080A] border border-white/[0.08]">
                                        <button
                                            onClick={() => setAppViewLayout('grid')}
                                            title="Squircle Launchpad Grid"
                                            className={`p-1.5 rounded-[6px] transition-colors ${appViewLayout === 'grid'
                                                    ? 'bg-white text-[#050506]'
                                                    : 'text-[#71717A] hover:text-white'
                                                }`}
                                        >
                                            <Grid className="w-3.5 h-3.5" />
                                        </button>
                                        <button
                                            onClick={() => setAppViewLayout('table')}
                                            title="Technical Package List"
                                            className={`p-1.5 rounded-[6px] transition-colors ${appViewLayout === 'table'
                                                    ? 'bg-white text-[#050506]'
                                                    : 'text-[#71717A] hover:text-white'
                                                }`}
                                        >
                                            <List className="w-3.5 h-3.5" />
                                        </button>
                                    </div>

                                    <button
                                        onClick={handleRefreshApps}
                                        className="px-3 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#1E1E24] border border-white/[0.08] text-[12px] font-medium text-white flex items-center gap-1.5 transition-colors"
                                    >
                                        <RotateCw
                                            className={`w-3.5 h-3.5 ${isRefreshingApps ? 'animate-spin' : ''}`}
                                        />
                                        <span>Refresh</span>
                                    </button>
                                </div>
                            </BentoSurface>

                            {/* 2. RECENT SECTION */}
                            {!appSearch && appCategory === 'All 96' && (
                                <section className="space-y-2.5">
                                    <div className="px-1 flex items-center justify-between">
                                        <h2 className="text-[12px] font-mono font-semibold uppercase tracking-[0.14em] text-[#71717A]">
                                            Recent
                                        </h2>
                                        <span className="text-[11px] font-mono text-[#71717A]">
                                            Instant Continuity Stream
                                        </span>
                                    </div>

                                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3.5">
                                        <BentoSurface className="col-span-1 sm:col-span-2 lg:col-span-1 p-4 hover:border-white/25 transition-all group cursor-pointer">
                                            <div
                                                onClick={() => setActiveMirroredApp(ALL_APPS_DATA[0])}
                                                className="flex items-center justify-between gap-3"
                                            >
                                                <div className="flex items-center gap-3.5 min-w-0">
                                                    <div className="w-12 h-12 rounded-[22%] bg-gradient-to-b from-white to-[#D4D4D8] text-[#050506] flex items-center justify-center font-mono text-[13px] font-bold shadow-[0_6px_20px_rgba(255,255,255,0.18)] shrink-0">
                                                        ((◉))
                                                    </div>
                                                    <div className="min-w-0">
                                                        <div className="flex items-center gap-2">
                                                            <span className="text-[15px] font-semibold text-white truncate">
                                                                Echo Music
                                                            </span>
                                                            <span className="px-1.5 py-0.5 rounded bg-white/[0.08] text-[10px] font-mono text-white/80">
                                                                RECENT
                                                            </span>
                                                        </div>
                                                        <div className="text-[12px] text-[#A1A1AA] mt-0.5 truncate">
                                                            Last streamed · Ready to launch
                                                        </div>
                                                    </div>
                                                </div>

                                                <span className="px-3 py-1.5 rounded-[8px] bg-[#16161A] group-hover:bg-white group-hover:text-[#050506] text-[12px] font-medium text-white border border-white/[0.08] transition-colors shrink-0">
                                                    Launch
                                                </span>
                                            </div>
                                        </BentoSurface>

                                        {ALL_APPS_DATA.filter(
                                            (a) => a.id === 'claude' || a.id === 'openspace'
                                        ).map((app) => (
                                            <BentoSurface
                                                key={app.id}
                                                className="p-4 hover:border-white/25 transition-all group cursor-pointer"
                                            >
                                                <div
                                                    onClick={() => setActiveMirroredApp(app)}
                                                    className="flex items-center justify-between gap-3"
                                                >
                                                    <div className="flex items-center gap-3.5 min-w-0">
                                                        <div className="w-12 h-12 rounded-[22%] bg-[#18181E] border border-white/[0.12] text-white flex items-center justify-center font-mono text-[15px] font-bold shrink-0">
                                                            {app.glyph}
                                                        </div>
                                                        <div className="min-w-0">
                                                            <div className="text-[15px] font-semibold text-white truncate">
                                                                {app.name}
                                                            </div>
                                                            <div className="text-[12px] text-[#71717A] mt-0.5 truncate">
                                                                {app.category} · v{app.version}
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <ArrowUpRight className="w-4 h-4 text-[#71717A] group-hover:text-white transition-colors shrink-0" />
                                                </div>
                                            </BentoSurface>
                                        ))}
                                    </div>
                                </section>
                            )}

                            {/* 3. ALL APPS GRID OR TABLE */}
                            <section className="space-y-3">
                                <div className="px-1 flex items-center justify-between">
                                    <h2 className="text-[12px] font-mono font-semibold uppercase tracking-[0.14em] text-[#71717A]">
                                        All apps ({filteredApps.length})
                                    </h2>
                                    <span className="text-[11.5px] font-mono text-[#71717A]">
                                        Select any package to open remote stream session
                                    </span>
                                </div>

                                {filteredApps.length === 0 ? (
                                    <BentoSurface className="py-16 text-center">
                                        <p className="text-[15px] font-medium text-white">
                                            No applications matching "{appSearch}"
                                        </p>
                                        <p className="text-[13px] text-[#71717A] mt-1">
                                            Reset the search query or switch category filter.
                                        </p>
                                    </BentoSurface>
                                ) : appViewLayout === 'grid' ? (
                                    <BentoSurface className="p-5 sm:p-6">
                                        <div className="grid grid-cols-3 sm:grid-cols-5 md:grid-cols-7 lg:grid-cols-9 gap-y-6 gap-x-3">
                                            {filteredApps.map((app) => {
                                                const toneClass =
                                                    iconFinishMode === 'monochrome' ? app.monoTone : app.studioTone;
                                                return (
                                                    <button
                                                        key={app.id}
                                                        onClick={() => setActiveMirroredApp(app)}
                                                        className="group flex flex-col items-center text-center p-1.5 rounded-[12px] hover:bg-white/[0.04] active:scale-95 transition-all duration-150 focus:outline-none"
                                                    >
                                                        <div
                                                            className={`relative w-[56px] h-[56px] rounded-[22%] bg-gradient-to-b ${toneClass} border ${app.lightTile && iconFinishMode === 'monochrome'
                                                                    ? 'border-white/40 text-[#050506]'
                                                                    : 'border-white/[0.12] text-white'
                                                                } flex items-center justify-center font-mono text-[13px] font-bold tracking-tight shadow-[0_8px_20px_rgba(0,0,0,0.6),inset_0_1px_0_rgba(255,255,255,0.2)] group-hover:border-white/40 group-hover:scale-105 transition-all`}
                                                        >
                                                            {app.glyph}
                                                        </div>
                                                        <span className="mt-2 text-[11.5px] font-medium leading-[1.25] text-[#D4D4D8] group-hover:text-white line-clamp-2 max-w-[84px]">
                                                            {app.name}
                                                        </span>
                                                    </button>
                                                );
                                            })}
                                        </div>
                                    </BentoSurface>
                                ) : (
                                    <BentoSurface className="divide-y divide-white/[0.06]">
                                        {filteredApps.map((app) => (
                                            <div
                                                key={app.id}
                                                onClick={() => setActiveMirroredApp(app)}
                                                className="px-5 py-3 flex items-center justify-between gap-4 hover:bg-[#16161A] transition-colors cursor-pointer group"
                                            >
                                                <div className="flex items-center gap-3.5 min-w-0">
                                                    <div className="w-9 h-9 rounded-[22%] bg-[#18181E] border border-white/[0.12] flex items-center justify-center font-mono text-[11px] font-bold text-white shrink-0">
                                                        {app.glyph}
                                                    </div>
                                                    <div className="min-w-0">
                                                        <div className="text-[13.5px] font-medium text-white truncate">
                                                            {app.name}
                                                        </div>
                                                        <div className="text-[11.5px] font-mono text-[#71717A]">
                                                            {app.category} · Package v{app.version}
                                                        </div>
                                                    </div>
                                                </div>

                                                <div className="flex items-center gap-3 shrink-0">
                                                    <span className="text-[11px] font-mono text-[#71717A] hidden sm:inline">
                                                        120Hz Stream Ready
                                                    </span>
                                                    <span className="px-3 py-1 rounded-[7px] bg-[#16161A] group-hover:bg-white group-hover:text-[#050506] border border-white/[0.08] text-[12px] font-medium text-white transition-colors">
                                                        Stream
                                                    </span>
                                                </div>
                                            </div>
                                        ))}
                                    </BentoSurface>
                                )}
                            </section>
                        </div>
                    )}

                    {/* ===============================================================
              VIEW C: DISPLAY (CROSS CONTROL & SPATIAL STUDIO)
          =============================================================== */}
                    {activeTab === 'display' && (
                        <div className="max-w-[940px] mx-auto space-y-6">
                            {/* 1. Header & Connection Banner */}
                            <div className="space-y-4">
                                <div>
                                    <h1 className="text-[26px] font-semibold tracking-[-0.025em] text-white">
                                        Cross Control
                                    </h1>
                                    <p className="text-[14px] text-[#A1A1AA] mt-1">
                                        Use your phone next to this PC: share this PC's mouse and keyboard with it,
                                        or use it as a second display.
                                    </p>
                                </div>

                                <BentoSurface className="p-4 sm:px-5 flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-[#111115]">
                                    <div className="space-y-1">
                                        <div className="flex items-center gap-2 text-[13.5px] font-semibold text-white">
                                            <span
                                                className={`w-2 h-2 rounded-full ${isConnected ? 'bg-white shadow-[0_0_8px_#FFFFFF]' : 'bg-[#71717A]'
                                                    }`}
                                            />
                                            <span>
                                                {isConnected
                                                    ? `Cross Control active with ${deviceName}.`
                                                    : 'Connect the phone to use Cross Control.'}
                                            </span>
                                        </div>
                                        <p className="text-[12.5px] text-[#A1A1AA]">
                                            Push the pointer back out of the phone, or press{' '}
                                            <kbd className="px-1.5 py-0.5 rounded bg-white/[0.08] border border-white/[0.1] font-mono text-[11px] text-white">
                                                Ctrl+Alt+Esc
                                            </kbd>
                                            , to return to this PC.
                                        </p>
                                    </div>

                                    {!isConnected && (
                                        <button
                                            onClick={handleToggleConnect}
                                            className="px-4 py-2 rounded-[9px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[12.5px] font-semibold shrink-0 self-start sm:self-center shadow-sm"
                                        >
                                            Connect Now
                                        </button>
                                    )}
                                </BentoSurface>
                            </div>

                            {/* 2. THREE CONTINUITY MODE SELECTOR CARDS */}
                            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                {[
                                    {
                                        id: 'mouse',
                                        title: 'Shared mouse and keyboard',
                                        subtitle: 'Move the pointer past the screen edge onto the phone',
                                        icon: MousePointer
                                    },
                                    {
                                        id: 'duplicate',
                                        title: 'Duplicate display',
                                        subtitle: "The phone shows this PC's screen",
                                        icon: Copy
                                    },
                                    {
                                        id: 'extend',
                                        title: 'Extend display',
                                        subtitle: 'The phone becomes a second screen',
                                        icon: Monitor
                                    }
                                ].map((mode) => {
                                    const Icon = mode.icon;
                                    const isSelected = crossControlMode === mode.id;
                                    return (
                                        <button
                                            key={mode.id}
                                            onClick={() => {
                                                setCrossControlMode(mode.id);
                                                setPhoneAsDisplayOn(mode.id !== 'mouse');
                                                triggerToast(
                                                    mode.title,
                                                    isConnected
                                                        ? `Applied to ${deviceName}`
                                                        : 'Preference saved · Connect phone to engage'
                                                );
                                            }}
                                            className={`group rounded-[14px] p-5 text-left flex flex-col justify-between gap-5 transition-all duration-200 border ${isSelected
                                                    ? 'bg-[#16161B] border-white shadow-[0_14px_35px_rgba(0,0,0,0.75),inset_0_1px_0_rgba(255,255,255,0.2)]'
                                                    : 'bg-[#0E0E11] hover:bg-[#131317] border-white/[0.07]'
                                                }`}
                                        >
                                            <div className="flex items-center justify-between w-full">
                                                <div
                                                    className={`w-10 h-10 rounded-[10px] flex items-center justify-center transition-colors ${isSelected
                                                            ? 'bg-white text-[#050506] shadow-[0_0_20px_rgba(255,255,255,0.25)]'
                                                            : 'bg-[#18181D] text-[#A1A1AA] border border-white/[0.08]'
                                                        }`}
                                                >
                                                    <Icon className="w-5 h-5" />
                                                </div>

                                                <span
                                                    className={`w-4 h-4 rounded-full border flex items-center justify-center ${isSelected ? 'border-white bg-white' : 'border-white/25'
                                                        }`}
                                                >
                                                    {isSelected && (
                                                        <span className="w-1.5 h-1.5 rounded-full bg-[#050506]" />
                                                    )}
                                                </span>
                                            </div>

                                            <div>
                                                <div className="text-[15px] font-semibold tracking-[-0.01em] text-white">
                                                    {mode.title}
                                                </div>
                                                <p className="text-[12.5px] text-[#A1A1AA] mt-1 leading-relaxed">
                                                    {mode.subtitle}
                                                </p>
                                            </div>
                                        </button>
                                    );
                                })}
                            </div>

                            {/* 3. INTERACTIVE SPATIAL DISPLAY ARRANGEMENT CANVAS ("Phone position") */}
                            <section className="space-y-3">
                                <div className="flex flex-col sm:flex-row sm:items-end justify-between gap-3">
                                    <div>
                                        <h2 className="text-[16px] font-semibold tracking-[-0.015em] text-white">
                                            Phone position
                                        </h2>
                                        <p className="text-[13px] text-[#A1A1AA] mt-0.5 max-w-[640px]">
                                            Drag the phone to the side of the screen where it sits. Extend display puts
                                            the phone's screen there, and shared mouse and keyboard crosses over along
                                            the highlighted part of that edge.
                                        </p>
                                    </div>

                                    <div className="flex items-center gap-1 p-1 rounded-[10px] bg-[#0E0E11] border border-white/[0.08] shrink-0 self-start">
                                        {[
                                            { id: 'left', label: 'Left Edge' },
                                            { id: 'top', label: 'Top Edge' },
                                            { id: 'bottom', label: 'Bottom Edge' },
                                            { id: 'right', label: 'Right Edge' }
                                        ].map((edge) => (
                                            <button
                                                key={edge.id}
                                                onClick={() => {
                                                    setPhonePosition(edge.id);
                                                    setPhoneOffset(0);
                                                }}
                                                className={`px-2.5 py-1 rounded-[7px] text-[11.5px] font-mono transition-all ${phonePosition === edge.id
                                                        ? 'bg-white text-[#050506] font-semibold shadow-sm'
                                                        : 'text-[#A1A1AA] hover:text-white'
                                                    }`}
                                            >
                                                {edge.label}
                                            </button>
                                        ))}
                                    </div>
                                </div>

                                {/* Architectural Dot-Grid Studio Canvas */}
                                <div
                                    ref={arrangementCanvasRef}
                                    onPointerMove={handleArrangementPointerMove}
                                    className="relative h-[300px] w-full rounded-[14px] bg-[#0A0A0D] border border-white/[0.08] shadow-inner overflow-hidden flex items-center justify-center select-none"
                                    style={{
                                        backgroundImage:
                                            'radial-gradient(rgba(255, 255, 255, 0.1) 1px, transparent 1px)',
                                        backgroundSize: '20px 20px'
                                    }}
                                >
                                    <div className="absolute top-3.5 left-4 flex items-center gap-2 text-[11px] font-mono text-[#71717A]">
                                        <span>SPATIAL CONTINUITY MATRIX</span>
                                        <span>·</span>
                                        <span className="text-white/80">Drag Phone node to calibrate seam</span>
                                    </div>

                                    {/* Central This PC Node + Draggable Phone Node */}
                                    <div className="relative flex items-center justify-center">
                                        <div className="relative w-[224px] h-[138px] rounded-[11px] bg-[#141418] border border-white/30 shadow-[0_24px_60px_rgba(0,0,0,0.85)] flex flex-col items-center justify-center">
                                            <div className="text-[14px] font-semibold text-white tracking-tight">
                                                This PC
                                            </div>
                                            <div className="text-[10.5px] font-mono text-[#71717A] mt-0.5">
                                                Primary Workspace
                                            </div>

                                            {phonePosition === 'right' && (
                                                <div
                                                    style={{ transform: `translateY(${phoneOffset}px)` }}
                                                    className="absolute -right-[3px] w-[4px] h-[78px] rounded-full bg-white shadow-[0_0_18px_#FFFFFF]"
                                                />
                                            )}
                                            {phonePosition === 'left' && (
                                                <div
                                                    style={{ transform: `translateY(${phoneOffset}px)` }}
                                                    className="absolute -left-[3px] w-[4px] h-[78px] rounded-full bg-white shadow-[0_0_18px_#FFFFFF]"
                                                />
                                            )}
                                            {phonePosition === 'top' && (
                                                <div
                                                    style={{ transform: `translateX(${phoneOffset}px)` }}
                                                    className="absolute -top-[3px] h-[4px] w-[48px] rounded-full bg-white shadow-[0_0_18px_#FFFFFF]"
                                                />
                                            )}
                                            {phonePosition === 'bottom' && (
                                                <div
                                                    style={{ transform: `translateX(${phoneOffset}px)` }}
                                                    className="absolute -bottom-[3px] h-[4px] w-[48px] rounded-full bg-white shadow-[0_0_18px_#FFFFFF]"
                                                />
                                            )}
                                        </div>

                                        <div
                                            onPointerDown={(e) => {
                                                e.currentTarget.setPointerCapture(e.pointerId);
                                                setIsDraggingPhone(true);
                                            }}
                                            style={{
                                                transform:
                                                    phonePosition === 'right'
                                                        ? `translate(144px, ${phoneOffset}px)`
                                                        : phonePosition === 'left'
                                                            ? `translate(-144px, ${phoneOffset}px)`
                                                            : phonePosition === 'top'
                                                                ? `translate(${phoneOffset}px, -118px)`
                                                                : `translate(${phoneOffset}px, 118px)`
                                            }}
                                            className={`absolute w-[46px] h-[90px] rounded-[9px] bg-[#1E1E24] border-2 border-white shadow-[0_12px_35px_rgba(255,255,255,0.22)] flex flex-col items-center justify-center cursor-grab active:cursor-grabbing transition-transform ${isDraggingPhone ? 'duration-0 scale-105' : 'duration-200'
                                                }`}
                                        >
                                            <span className="text-[11px] font-semibold text-white tracking-tight">
                                                Phone
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <div className="px-1 flex items-center justify-between text-[12.5px] text-[#A1A1AA]">
                                    <span>{phonePositionCaption}</span>
                                    <span className="font-mono text-[11px] text-[#71717A]">
                                        Offset: {phoneOffset}px
                                    </span>
                                </div>
                            </section>

                            {/* 4. BOTTOM APPLE INSET GROUP CARD: "Phone as display" */}
                            <BentoSurface className="p-5 sm:px-6 flex items-center justify-between gap-4">
                                <div className="flex items-center gap-4">
                                    <div className="w-10 h-10 rounded-[10px] bg-[#16161A] border border-white/[0.08] flex items-center justify-center text-white">
                                        <Monitor className="w-5 h-5" />
                                    </div>
                                    <div>
                                        <div className="text-[15.5px] font-semibold text-white">
                                            Phone as display
                                        </div>
                                        <div className="text-[13px] text-[#A1A1AA] mt-0.5">
                                            {phoneAsDisplayOn
                                                ? `Active · ${crossControlMode === 'duplicate'
                                                    ? 'Mirroring Primary Screen'
                                                    : 'Extended 120Hz Virtual Display'
                                                }`
                                                : 'Off'}
                                        </div>
                                    </div>
                                </div>

                                <MonochromeToggle
                                    checked={phoneAsDisplayOn}
                                    onChange={(val) => {
                                        setPhoneAsDisplayOn(val);
                                        if (val && crossControlMode === 'mouse') {
                                            setCrossControlMode('extend');
                                        } else if (!val) {
                                            setCrossControlMode('mouse');
                                        }
                                        triggerToast(
                                            val ? 'Phone as Display Enabled' : 'Phone as Display Off',
                                            val ? '2400×1080 @ 120Hz Virtual Sink' : 'Reverted to pointer sharing'
                                        );
                                    }}
                                    label="Phone as display"
                                />
                            </BentoSurface>
                        </div>
                    )}

                    {/* ===============================================================
              VIEW D: FILES (MACOS SYSTEM SETTINGS STORAGE MANAGER)
          =============================================================== */}
                    {activeTab === 'files' && (
                        <div className="max-w-[900px] mx-auto space-y-7">
                            {/* 1. HERO STORAGE MOUNT BANNER */}
                            <BentoSurface className="p-6 flex flex-col sm:flex-row sm:items-center justify-between gap-5">
                                <div className="flex items-center gap-4">
                                    <div className="w-12 h-12 rounded-[12px] bg-white text-[#050506] flex items-center justify-center shadow-[0_0_25px_rgba(255,255,255,0.2)] shrink-0">
                                        <Folder className="w-6 h-6" />
                                    </div>
                                    <div>
                                        <h1 className="text-[19px] font-semibold tracking-[-0.02em] text-white">
                                            Phone storage
                                        </h1>
                                        <p className="text-[13.5px] text-[#A1A1AA] mt-0.5">
                                            {isStorageMounted
                                                ? `Mounted read-only at ${mountLocation.replace(' (default)', '')} · 142.4 GB free`
                                                : 'Connect your phone to mount its storage.'}
                                        </p>
                                    </div>
                                </div>

                                <div className="flex items-center gap-2.5 self-start sm:self-center">
                                    {!isConnected && (
                                        <button
                                            onClick={handleToggleConnect}
                                            className="px-3.5 py-2 rounded-[9px] bg-[#16161A] hover:bg-[#1E1E24] border border-white/[0.1] text-[12.5px] font-medium text-white transition-colors"
                                        >
                                            Connect Phone
                                        </button>
                                    )}
                                    <button
                                        disabled={!isConnected}
                                        onClick={() => {
                                            setIsStorageMounted(!isStorageMounted);
                                            triggerToast(
                                                !isStorageMounted ? 'Phone Storage Mounted' : 'Storage Unmounted',
                                                mountLocation
                                            );
                                        }}
                                        className={`px-5 py-2 rounded-[9px] text-[13px] font-semibold transition-all ${!isConnected
                                                ? 'bg-[#16161A] text-[#52525B] border border-white/[0.05] cursor-not-allowed'
                                                : isStorageMounted
                                                    ? 'bg-[#16161A] hover:bg-[#22222A] text-white border border-white/[0.15]'
                                                    : 'bg-white hover:bg-[#E4E4E7] text-[#050506] shadow-sm'
                                            }`}
                                    >
                                        {isStorageMounted ? 'Unmount' : 'Mount'}
                                    </button>
                                </div>
                            </BentoSurface>

                            {/* 2. SECTION 1: STORAGE */}
                            <section className="space-y-2.5">
                                <div className="px-1">
                                    <h2 className="text-[15.5px] font-semibold text-white">Storage</h2>
                                    <p className="text-[13px] text-[#A1A1AA] mt-0.5">
                                        The phone's storage is mounted read-only.
                                    </p>
                                </div>

                                <BentoSurface className="divide-y divide-white/[0.06]">
                                    <div className="p-4 sm:px-6 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">Mount location</div>
                                            <div className="mt-1 inline-flex items-center px-2.5 py-0.5 rounded-[6px] bg-[#16161A] border border-white/[0.06] text-[12px] font-mono text-[#A1A1AA]">
                                                {mountLocation}
                                            </div>
                                        </div>
                                        <button
                                            onClick={() => openPathModal('mount')}
                                            className="px-4 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#22222A] border border-white/[0.1] text-[13px] font-medium text-white transition-colors self-start sm:self-center"
                                        >
                                            Change...
                                        </button>
                                    </div>

                                    <div className="p-4 sm:px-6 flex items-center justify-between gap-4">
                                        <div className="text-[14.5px] font-medium text-white">
                                            Mount automatically when connected
                                        </div>
                                        <MonochromeToggle
                                            checked={autoMount}
                                            onChange={(val) => {
                                                setAutoMount(val);
                                                triggerToast(
                                                    'Auto-Mount Preference Updated',
                                                    val ? 'Will mount automatically on connection' : 'Manual mount required'
                                                );
                                            }}
                                            label="Mount automatically when connected"
                                        />
                                    </div>
                                </BentoSurface>
                            </section>

                            {/* 3. SECTION 2: SHARE WITH THE PHONE */}
                            <section className="space-y-2.5">
                                <div className="px-1">
                                    <h2 className="text-[15.5px] font-semibold text-white">
                                        Share with the phone
                                    </h2>
                                    <p className="text-[13px] text-[#A1A1AA] mt-0.5">
                                        The phone sees the folder in its Files app and in Smart Connect's Files tab.
                                        It can open and copy files, but not change them.
                                    </p>
                                </div>

                                <BentoSurface className="divide-y divide-white/[0.06]">
                                    <div className="p-4 sm:px-6 flex items-center justify-between gap-4">
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">
                                                Share a folder with the phone
                                            </div>
                                            <div className="text-[12.5px] text-[#A1A1AA] mt-0.5">
                                                The phone can browse it in its Files app while it's connected
                                            </div>
                                        </div>
                                        <MonochromeToggle
                                            checked={shareFolderEnabled}
                                            onChange={setShareFolderEnabled}
                                            label="Share a folder with the phone"
                                        />
                                    </div>

                                    <div
                                        className={`p-4 sm:px-6 flex flex-col sm:flex-row sm:items-center justify-between gap-3 transition-opacity ${!shareFolderEnabled ? 'opacity-40 pointer-events-none' : ''
                                            }`}
                                    >
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">Shared folder</div>
                                            <div className="mt-1 inline-flex items-center px-2.5 py-0.5 rounded-[6px] bg-[#16161A] border border-white/[0.06] text-[12px] font-mono text-[#A1A1AA]">
                                                {sharedFolderPath}
                                            </div>
                                        </div>

                                        <div className="flex items-center gap-2 self-start sm:self-center">
                                            <button
                                                onClick={() =>
                                                    triggerToast(
                                                        'Opening Shared Directory',
                                                        sharedFolderPath.replace(' (default)', '')
                                                    )
                                                }
                                                title="Open in File Manager"
                                                className="p-2 rounded-[8px] bg-[#16161A] hover:bg-[#22222A] border border-white/[0.1] text-[#A1A1AA] hover:text-white transition-colors"
                                            >
                                                <FolderOpen className="w-4 h-4" />
                                            </button>
                                            <button
                                                onClick={() => openPathModal('shared')}
                                                className="px-4 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#22222A] border border-white/[0.1] text-[13px] font-medium text-white transition-colors"
                                            >
                                                Change...
                                            </button>
                                        </div>
                                    </div>

                                    <div
                                        className={`p-4 sm:px-6 flex items-center justify-between gap-4 transition-opacity ${!shareFolderEnabled ? 'opacity-40 pointer-events-none' : ''
                                            }`}
                                    >
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">
                                                Include hidden files
                                            </div>
                                            <div className="text-[12.5px] text-[#A1A1AA] mt-0.5">
                                                Files and folders whose names start with a dot, such as ~/.ssh
                                            </div>
                                        </div>
                                        <MonochromeToggle
                                            checked={includeHiddenFiles}
                                            onChange={setIncludeHiddenFiles}
                                            label="Include hidden files"
                                        />
                                    </div>

                                    <div
                                        className={`p-4 sm:px-6 flex items-center justify-between gap-4 transition-opacity ${!shareFolderEnabled ? 'opacity-40 pointer-events-none' : ''
                                            }`}
                                    >
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">
                                                Let the phone delete files
                                            </div>
                                            <div className="text-[12.5px] text-[#A1A1AA] mt-0.5">
                                                Deleted files go to the Trash, so you can restore them
                                            </div>
                                        </div>
                                        <MonochromeToggle
                                            checked={allowDeleteFiles}
                                            onChange={setAllowDeleteFiles}
                                            label="Let the phone delete files"
                                        />
                                    </div>

                                    <div className="p-4 sm:px-6 flex items-center justify-between gap-4">
                                        <div>
                                            <div className="text-[14.5px] font-medium text-white">Status</div>
                                            <div className="text-[12.5px] text-[#A1A1AA] mt-0.5">
                                                {!shareFolderEnabled
                                                    ? 'Sharing disabled'
                                                    : isConnected
                                                        ? 'Active · Mounted via SFTP'
                                                        : 'Starts when the phone connects'}
                                            </div>
                                        </div>

                                        <span
                                            className={`px-2.5 py-1 rounded-[7px] text-[11px] font-mono font-semibold ${shareFolderEnabled && isConnected
                                                    ? 'bg-white text-[#050506]'
                                                    : 'bg-[#16161A] text-[#71717A] border border-white/[0.07]'
                                                }`}
                                        >
                                            {shareFolderEnabled && isConnected ? 'ACTIVE' : 'STANDBY'}
                                        </span>
                                    </div>
                                </BentoSurface>
                            </section>
                        </div>
                    )}

                    {/* ===============================================================
              VIEW E: NOTIFICATIONS (MONOCHROME NOTIFICATION CENTER)
          =============================================================== */}
                    {activeTab === 'notifications' && (
                        <div className="max-w-[820px] mx-auto h-full flex flex-col">
                            {/* Top Header + Mode Switcher */}
                            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                                <div>
                                    <h1 className="text-[22px] font-semibold tracking-[-0.025em] text-white">
                                        Notifications
                                    </h1>
                                    <p className="text-[13px] text-[#A1A1AA]">
                                        Real-time Continuity alert relay from {deviceName}
                                    </p>
                                </div>

                                <div className="flex items-center gap-1 p-1 rounded-[10px] bg-[#0E0E11] border border-white/[0.08] self-start sm:self-center">
                                    <button
                                        onClick={() => setNotificationViewMode('empty')}
                                        className={`px-3 py-1.5 rounded-[7px] text-[12px] font-medium transition-all ${notificationViewMode === 'empty'
                                                ? 'bg-white text-[#050506] font-semibold'
                                                : 'text-[#A1A1AA] hover:text-white'
                                            }`}
                                    >
                                        Empty State (MVP)
                                    </button>
                                    <button
                                        onClick={() => setNotificationViewMode('live')}
                                        className={`px-3 py-1.5 rounded-[7px] text-[12px] font-medium transition-all flex items-center gap-1.5 ${notificationViewMode === 'live'
                                                ? 'bg-white text-[#050506] font-semibold'
                                                : 'text-[#A1A1AA] hover:text-white'
                                            }`}
                                    >
                                        <span>Simulate Live Feed</span>
                                        <span
                                            className={`px-1.5 py-0.2 rounded text-[10px] font-mono ${notificationViewMode === 'live'
                                                    ? 'bg-[#050506] text-white'
                                                    : 'bg-white/10 text-white'
                                                }`}
                                        >
                                            {notifications.length}
                                        </span>
                                    </button>
                                </div>
                            </div>

                            {/* STATE 1: ARCHITECTURAL MONOCHROME EMPTY STATE */}
                            {notificationViewMode === 'empty' ? (
                                <BentoSurface className="flex-1 min-h-[450px] flex flex-col items-center justify-center text-center p-8">
                                    <div className="relative mb-6">
                                        <div className="w-24 h-24 rounded-[26px] bg-gradient-to-b from-[#1C1C22] to-[#0E0E12] border border-white/[0.12] shadow-[0_24px_60px_rgba(0,0,0,0.85),inset_0_1px_0_rgba(255,255,255,0.18)] flex items-center justify-center">
                                            <svg
                                                className="w-11 h-11 text-white/85"
                                                viewBox="0 0 48 48"
                                                fill="none"
                                                xmlns="http://www.w3.org/2000/svg"
                                            >
                                                <path
                                                    d="M16 14H36C38.2091 14 40 15.7909 40 18V30C40 32.2091 38.2091 34 36 34H29L24 39L19 34H14C11.7909 34 10 32.2091 10 30V20"
                                                    stroke="currentColor"
                                                    strokeWidth="2.8"
                                                    strokeLinecap="round"
                                                    strokeLinejoin="round"
                                                />
                                                <circle
                                                    cx="13"
                                                    cy="13"
                                                    r="7"
                                                    className="fill-[#141418] stroke-white"
                                                    strokeWidth="2.8"
                                                />
                                                <path
                                                    d="M13 9.5V13.5"
                                                    stroke="white"
                                                    strokeWidth="2.8"
                                                    strokeLinecap="round"
                                                />
                                                <circle cx="13" cy="16.5" r="1.4" fill="white" />
                                            </svg>
                                        </div>
                                    </div>

                                    <h2 className="text-[24px] font-semibold tracking-[-0.025em] text-white">
                                        No Notifications
                                    </h2>
                                    <p className="text-[14px] text-[#A1A1AA] max-w-[390px] mt-1.5 leading-relaxed">
                                        Notifications from the phone show up here while it is connected.
                                    </p>

                                    <div className="mt-7 flex flex-wrap items-center justify-center gap-3">
                                        {!isConnected && (
                                            <button
                                                onClick={handleToggleConnect}
                                                className="px-5 py-2.5 rounded-[10px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[13px] font-semibold shadow-sm transition-all"
                                            >
                                                Connect Device
                                            </button>
                                        )}
                                        <button
                                            onClick={() => setNotificationViewMode('live')}
                                            className="px-5 py-2.5 rounded-[10px] bg-[#16161A] hover:bg-[#1E1E24] border border-white/[0.1] text-[13px] font-medium text-white transition-all"
                                        >
                                            Simulate Live Feed
                                        </button>
                                    </div>
                                </BentoSurface>
                            ) : (
                                /* STATE 2: SIMULATED LIVE MONOCHROME NOTIFICATION FEED */
                                <div className="space-y-3">
                                    <div className="px-1 flex items-center justify-between">
                                        <span className="text-[11px] font-mono uppercase tracking-[0.14em] text-[#71717A]">
                                            LIVE CONTINUITY RELAY ({notifications.length})
                                        </span>
                                        {notifications.length > 0 && (
                                            <button
                                                onClick={() => {
                                                    setNotifications([]);
                                                    setNotificationViewMode('empty');
                                                }}
                                                className="text-[12px] font-mono text-[#A1A1AA] hover:text-white"
                                            >
                                                Clear All
                                            </button>
                                        )}
                                    </div>

                                    {notifications.map((item) => (
                                        <BentoSurface
                                            key={item.id}
                                            className="p-4 sm:p-5 hover:border-white/20 transition-all"
                                        >
                                            <div className="flex items-start justify-between gap-4">
                                                <div className="flex items-start gap-3.5 min-w-0">
                                                    <div className="w-10 h-10 rounded-[10px] bg-[#18181E] border border-white/[0.12] flex items-center justify-center font-mono text-[12px] font-bold text-white shrink-0 mt-0.5">
                                                        {item.glyph}
                                                    </div>
                                                    <div className="min-w-0">
                                                        <div className="flex items-center gap-2 flex-wrap">
                                                            <span className="text-[11.5px] font-mono font-semibold uppercase tracking-wider text-white/80">
                                                                {item.app}
                                                            </span>
                                                            <span className="px-1.5 py-0.2 rounded bg-white/[0.06] text-[10px] font-mono text-[#A1A1AA]">
                                                                {item.tag}
                                                            </span>
                                                            <span className="text-[11px] font-mono text-[#71717A]">
                                                                · {item.time}
                                                            </span>
                                                            {item.unread && (
                                                                <span className="w-2 h-2 rounded-full bg-white" />
                                                            )}
                                                        </div>
                                                        <h3 className="text-[15px] font-semibold text-white mt-1">
                                                            {item.title}
                                                        </h3>
                                                        <p className="text-[13px] text-[#A1A1AA] mt-0.5 leading-relaxed">
                                                            {item.body}
                                                        </p>
                                                    </div>
                                                </div>

                                                <div className="flex items-center gap-1.5 shrink-0">
                                                    <button
                                                        onClick={() =>
                                                            setReplyingToId(replyingToId === item.id ? null : item.id)
                                                        }
                                                        className="px-3 py-1 rounded-[7px] bg-[#16161A] hover:bg-white hover:text-[#050506] border border-white/[0.08] text-[12px] font-medium text-white transition-colors"
                                                    >
                                                        Reply
                                                    </button>
                                                    <button
                                                        onClick={() =>
                                                            setNotifications((prev) => prev.filter((n) => n.id !== item.id))
                                                        }
                                                        title="Dismiss"
                                                        className="p-1.5 rounded-[7px] text-[#71717A] hover:text-white hover:bg-[#16161A] transition-colors"
                                                    >
                                                        <X className="w-4 h-4" />
                                                    </button>
                                                </div>
                                            </div>

                                            {replyingToId === item.id && (
                                                <form
                                                    onSubmit={(e) => {
                                                        e.preventDefault();
                                                        if (!replyText.trim()) return;
                                                        triggerToast(`Reply Relayed to ${item.app}`, `"${replyText}"`);
                                                        setReplyText('');
                                                        setReplyingToId(null);
                                                    }}
                                                    className="mt-3.5 pt-3.5 border-t border-white/[0.07] flex items-center gap-2"
                                                >
                                                    <input
                                                        type="text"
                                                        value={replyText}
                                                        onChange={(e) => setReplyText(e.target.value)}
                                                        placeholder={`Reply to ${item.title} via ${deviceName}…`}
                                                        autoFocus
                                                        className="flex-1 px-3 py-1.5 rounded-[8px] bg-[#050506] border border-white/[0.15] text-[13px] text-white placeholder-[#71717A] focus:outline-none focus:border-white"
                                                    />
                                                    <button
                                                        type="submit"
                                                        className="px-3.5 py-1.5 rounded-[8px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[12.5px] font-semibold flex items-center gap-1.5"
                                                    >
                                                        <Send className="w-3.5 h-3.5" />
                                                        <span>Send</span>
                                                    </button>
                                                </form>
                                            )}
                                        </BentoSurface>
                                    ))}
                                </div>
                            )}
                        </div>
                    )}
                </main>
            </div>

            {/* =====================================================================
          MODAL 1: ⌘K COMMAND PALETTE (APPLE PRO SPOTLIGHT)
      ===================================================================== */}
            {isCmdPaletteOpen && (
                <div className="fixed inset-0 z-50 bg-black/75 backdrop-blur-md flex items-start justify-center pt-[12vh] p-4">
                    <div className="w-full max-w-[580px] rounded-[16px] bg-[#0E0E11] border border-white/[0.15] shadow-[0_35px_100px_rgba(0,0,0,0.95),inset_0_1px_0_rgba(255,255,255,0.12)] overflow-hidden">
                        <div className="px-4 py-3.5 border-b border-white/[0.08] flex items-center gap-3">
                            <Command className="w-4 h-4 text-[#A1A1AA] shrink-0" />
                            <input
                                type="text"
                                value={cmdQuery}
                                onChange={(e) => setCmdQuery(e.target.value)}
                                placeholder="Type a command, jump to a view, or stream an app (e.g. Echo Music, Claude, Mount)..."
                                autoFocus
                                className="w-full bg-transparent text-[14px] text-white placeholder-[#71717A] focus:outline-none"
                            />
                            <button
                                onClick={() => setIsCmdPaletteOpen(false)}
                                className="px-1.5 py-0.5 rounded bg-[#16161A] border border-white/[0.08] text-[10.5px] font-mono text-[#A1A1AA]"
                            >
                                ESC
                            </button>
                        </div>

                        <div className="max-h-[340px] overflow-y-auto p-2 divide-y divide-white/[0.04]">
                            {commandActions.length === 0 ? (
                                <div className="py-10 text-center text-[13px] text-[#71717A]">
                                    No matching commands or installed packages.
                                </div>
                            ) : (
                                commandActions.map((item) => (
                                    <button
                                        key={item.id}
                                        onClick={() => {
                                            item.action();
                                            setIsCmdPaletteOpen(false);
                                        }}
                                        className="w-full px-3 py-2.5 rounded-[10px] hover:bg-white hover:text-[#050506] flex items-center justify-between gap-3 text-left transition-colors group"
                                    >
                                        <div className="min-w-0">
                                            <div className="text-[13.5px] font-medium truncate">{item.title}</div>
                                            <div className="text-[11px] font-mono text-[#71717A] group-hover:text-[#050506]/70">
                                                {item.category}
                                            </div>
                                        </div>
                                        <span className="px-2 py-0.5 rounded bg-[#16161A] group-hover:bg-[#050506] group-hover:text-white text-[10.5px] font-mono text-[#A1A1AA] shrink-0">
                                            {item.shortcut}
                                        </span>
                                    </button>
                                ))
                            )}
                        </div>
                    </div>
                </div>
            )}

            {/* =====================================================================
          MODAL 2: PURE UI REMOTE APPLICATION STREAM SESSION DIALOG
      ===================================================================== */}
            {activeMirroredApp && (
                <div className="fixed inset-0 z-50 bg-black/75 backdrop-blur-md flex items-center justify-center p-4">
                    <div className="w-full max-w-[440px] rounded-[16px] bg-[#0E0E11] border border-white/[0.15] shadow-[0_35px_100px_rgba(0,0,0,0.95)] overflow-hidden p-6 space-y-5">
                        <div className="flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <span
                                    className={`w-2 h-2 rounded-full ${isConnected ? 'bg-white shadow-[0_0_8px_#FFFFFF]' : 'bg-[#71717A]'
                                        }`}
                                />
                                <span className="text-[11.5px] font-mono uppercase tracking-[0.12em] text-[#A1A1AA]">
                                    Remote App Stream · {deviceName}
                                </span>
                            </div>
                            <button
                                onClick={() => setActiveMirroredApp(null)}
                                className="p-1.5 rounded-[8px] bg-[#16161A] hover:bg-white hover:text-[#050506] text-[#A1A1AA] transition-colors"
                            >
                                <X className="w-3.5 h-3.5" />
                            </button>
                        </div>

                        <div className="p-5 rounded-[12px] bg-[#08080A] border border-white/[0.07] flex items-center gap-4">
                            <div className="w-14 h-14 rounded-[22%] bg-[#18181F] border border-white/20 flex items-center justify-center font-mono text-[18px] font-bold text-white shrink-0">
                                {activeMirroredApp.glyph}
                            </div>
                            <div className="min-w-0">
                                <div className="text-[11px] font-mono uppercase tracking-[0.12em] text-[#71717A]">
                                    {activeMirroredApp.category} · v{activeMirroredApp.version}
                                </div>
                                <h3 className="text-[19px] font-semibold text-white truncate mt-0.5">
                                    {activeMirroredApp.name}
                                </h3>
                                <p className="text-[12.5px] text-[#A1A1AA] mt-0.5">
                                    {isConnected
                                        ? '120Hz Continuity Virtual Sink Ready'
                                        : 'Endpoint Disconnected · Handshake required'}
                                </p>
                            </div>
                        </div>

                        <p className="text-[13px] text-[#A1A1AA] leading-relaxed">
                            {isConnected
                                ? `Streaming ${activeMirroredApp.name} directly from ${deviceName} with shared desktop keyboard and pointer routing.`
                                : `Connect ${deviceName} to initiate a low-latency virtual display stream for ${activeMirroredApp.name}.`}
                        </p>

                        <div className="flex items-center justify-end gap-2.5 pt-1">
                            <button
                                onClick={() => setActiveMirroredApp(null)}
                                className="px-4 py-2 rounded-[9px] bg-[#16161A] hover:bg-[#1E1E24] text-[12.5px] font-medium text-[#A1A1AA] hover:text-white"
                            >
                                Close
                            </button>
                            {!isConnected && (
                                <button
                                    onClick={handleToggleConnect}
                                    className="px-5 py-2 rounded-[9px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[12.5px] font-semibold shadow-sm"
                                >
                                    Connect & Stream
                                </button>
                            )}
                        </div>
                    </div>
                </div>
            )}

            {/* =====================================================================
          MODAL 3: MONOCHROME PATH EDITOR MODAL (/home/lalitmukesh/Phone)
      ===================================================================== */}
            {editingPathTarget && (
                <div className="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-4">
                    <form
                        onSubmit={savePathModal}
                        className="w-full max-w-[440px] rounded-[16px] bg-[#0E0E11] border border-white/[0.15] shadow-[0_30px_80px_rgba(0,0,0,0.9)] p-6 space-y-4"
                    >
                        <div className="flex items-center gap-3">
                            <div className="w-9 h-9 rounded-[9px] bg-white text-[#050506] flex items-center justify-center">
                                <FolderOpen className="w-4 h-4" />
                            </div>
                            <div>
                                <h3 className="text-[15.5px] font-semibold text-white">
                                    {editingPathTarget === 'mount'
                                        ? 'Configure Mount Location'
                                        : 'Configure Shared Directory'}
                                </h3>
                                <p className="text-[12px] text-[#A1A1AA]">
                                    Enter local POSIX path on this workstation
                                </p>
                            </div>
                        </div>

                        <input
                            type="text"
                            value={tempPathValue}
                            onChange={(e) => setTempPathValue(e.target.value)}
                            autoFocus
                            className="w-full px-3.5 py-2 rounded-[9px] bg-[#050506] border border-white/25 font-mono text-[13px] text-white focus:outline-none focus:border-white"
                        />

                        <div className="flex items-center justify-end gap-2 pt-2">
                            <button
                                type="button"
                                onClick={() => setEditingPathTarget(null)}
                                className="px-4 py-1.5 rounded-[8px] bg-[#16161A] hover:bg-[#1E1E24] text-[12.5px] font-medium text-[#A1A1AA] hover:text-white"
                            >
                                Cancel
                            </button>
                            <button
                                type="submit"
                                className="px-4 py-1.5 rounded-[8px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[12.5px] font-semibold"
                            >
                                Save Path
                            </button>
                        </div>
                    </form>
                </div>
            )}

            {/* =====================================================================
          MODAL 4: SYSTEM TELEMETRY & SPECIFICATIONS SHEET
      ===================================================================== */}
            {showSpecsModal && (
                <div className="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-4">
                    <div className="w-full max-w-[440px] rounded-[16px] bg-[#0E0E11] border border-white/[0.15] shadow-[0_30px_80px_rgba(0,0,0,0.9)] p-6 space-y-4">
                        <div className="flex items-center justify-between">
                            <div className="flex items-center gap-2.5">
                                <Smartphone className="w-5 h-5 text-white" />
                                <h3 className="text-[16px] font-semibold text-white">Endpoint Telemetry</h3>
                            </div>
                            <button
                                onClick={() => setShowSpecsModal(false)}
                                className="p-1 rounded-full hover:bg-white/10 text-[#A1A1AA] hover:text-white"
                            >
                                <X className="w-4 h-4" />
                            </button>
                        </div>

                        <div className="rounded-[12px] bg-[#08080A] border border-white/[0.07] divide-y divide-white/[0.06] text-[12.5px] font-mono">
                            <div className="px-4 py-2.5 flex justify-between">
                                <span className="text-[#71717A]">ENDPOINT</span>
                                <span className="text-white">{deviceName}</span>
                            </div>
                            <div className="px-4 py-2.5 flex justify-between">
                                <span className="text-[#71717A]">STREAM PROTOCOL</span>
                                <span className="text-white">2400×1080 · 120Hz Sink</span>
                            </div>
                            <div className="px-4 py-2.5 flex justify-between">
                                <span className="text-[#71717A]">BATTERY TELEMETRY</span>
                                <span className="text-white">60% · 4310 mAh</span>
                            </div>
                            <div className="px-4 py-2.5 flex justify-between">
                                <span className="text-[#71717A]">MOUNT PATH</span>
                                <span className="text-[#E4E4E7]">/home/lalitmukesh/Phone</span>
                            </div>
                            <div className="px-4 py-2.5 flex justify-between">
                                <span className="text-[#71717A]">SHARED PATH</span>
                                <span className="text-[#E4E4E7]">/home/lalitmukesh</span>
                            </div>
                        </div>

                        <div className="flex justify-end">
                            <button
                                onClick={() => setShowSpecsModal(false)}
                                className="px-5 py-1.5 rounded-[8px] bg-white hover:bg-[#E4E4E7] text-[#050506] text-[12.5px] font-semibold"
                            >
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            )}

            {/* =====================================================================
          MONOCHROME FLOATING HUD TOAST
      ===================================================================== */}
            {toastMessage && (
                <div className="fixed bottom-6 right-6 z-50 flex items-center gap-3 px-4 py-3 rounded-[12px] bg-[#141418]/95 backdrop-blur-2xl border border-white/20 shadow-[0_20px_50px_rgba(0,0,0,0.85)]">
                    <div className="w-6 h-6 rounded-full bg-white text-[#050506] flex items-center justify-center shrink-0">
                        <Check className="w-3.5 h-3.5" />
                    </div>
                    <div className="pr-2">
                        <div className="text-[12.5px] font-semibold text-white leading-tight">
                            {toastMessage.title}
                        </div>
                        {toastMessage.subtitle && (
                            <div className="text-[11px] font-mono text-[#A1A1AA] mt-0.5">
                                {toastMessage.subtitle}
                            </div>
                        )}
                    </div>
                </div>
            )}
        </div>
    );
}